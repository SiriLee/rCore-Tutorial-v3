use core::sync::atomic::{AtomicUsize, Ordering};

use super::{PageTable, PhysAddr, PhysPageNum, PteFlags, VirtAddr, VirtPageNum, PAGE_SIZE};

const MEMORY_END: usize = 0x8800_0000;
const UART_BASE: usize = 0x1000_0000;
static KERNEL_TOKEN: AtomicUsize = AtomicUsize::new(0);

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss();
    fn ebss();
    fn ekernel();
}

/// 内核恒等映射的分段权限表。`start < end` 的区间必须页对齐且互不重叠。
fn kernel_sections() -> [(usize, usize, PteFlags); 5] {
    [
        (
            stext as *const () as usize,
            etext as *const () as usize,
            PteFlags::R | PteFlags::X,
        ),
        (
            srodata as *const () as usize,
            erodata as *const () as usize,
            PteFlags::R,
        ),
        (
            sdata as *const () as usize,
            edata as *const () as usize,
            PteFlags::R | PteFlags::W,
        ),
        (
            sbss as *const () as usize,
            ebss as *const () as usize,
            PteFlags::R | PteFlags::W,
        ),
        (
            ekernel as *const () as usize,
            MEMORY_END,
            PteFlags::R | PteFlags::W,
        ),
    ]
}

/// 断言所有边界按链接布局排列、页对齐，且严格递增。
/// 链接脚本被改坏时这里立刻失败，而不是静默建立一张错误的页表。
fn check_layout() -> Result<(), &'static str> {
    let bounds = [
        stext as *const () as usize,
        etext as *const () as usize,
        srodata as *const () as usize,
        erodata as *const () as usize,
        sdata as *const () as usize,
        edata as *const () as usize,
        sbss as *const () as usize,
        ebss as *const () as usize,
        ekernel as *const () as usize,
    ];
    for index in 0..bounds.len() - 1 {
        if bounds[index] > bounds[index + 1] {
            return Err("kernel-space init: kernel section bounds are out of order");
        }
    }
    for bound in bounds {
        if bound & (PAGE_SIZE - 1) != 0 {
            return Err("kernel-space init: kernel section bound is not page aligned");
        }
    }
    if ekernel as *const () as usize >= MEMORY_END {
        return Err("kernel-space init: kernel image does not fit in RAM");
    }
    Ok(())
}

/// 把 `[start, end)` 恒等映射到 `[start, end)`，即 VA 与 PA 相同。
/// 任一次映射失败立即返回 Err：绝不能发布一张不完整的页表。
fn map_identity_range(
    page_table: &mut PageTable,
    start: usize,
    end: usize,
    flags: PteFlags,
) -> Result<(), &'static str> {
    if start & (PAGE_SIZE - 1) != 0 || end & (PAGE_SIZE - 1) != 0 || start > end {
        return Err("kernel-space init: identity range is not page aligned");
    }

    let first = VirtAddr(start).floor().0;
    let last = VirtAddr(end).floor().0;
    for vpn in first..last {
        page_table.map(VirtPageNum(vpn), PhysPageNum(vpn), flags)?;
    }
    Ok(())
}

fn check_page(
    page_table: &PageTable,
    page_start: usize,
    readable: bool,
    writable: bool,
    executable: bool,
    what: &'static str,
) -> Result<(), &'static str> {
    let entry = page_table
        .translate(VirtAddr(page_start).floor())
        .ok_or("kernel-space init: representative page is not mapped")?;
    let flags = entry.flags();

    if flags.contains(PteFlags::U) {
        return Err("kernel-space init: kernel page must not be user accessible");
    }
    if flags.contains(PteFlags::R) != readable
        || flags.contains(PteFlags::W) != writable
        || flags.contains(PteFlags::X) != executable
    {
        return Err("kernel-space init: representative page has unexpected permissions");
    }
    let _ = what;
    Ok(())
}

/// hart 0 调用一次：建立内核恒等映射，做软件权限自检，然后以 Release 发布 satp token。
pub fn kernel_space_init() -> Result<(), &'static str> {
    check_layout()?;

    let mut page_table = PageTable::new().ok_or("kernel-space init: cannot allocate root page")?;

    for (start, end, flags) in kernel_sections() {
        map_identity_range(&mut page_table, start, end, flags)?;
    }
    map_identity_range(
        &mut page_table,
        UART_BASE,
        UART_BASE + PAGE_SIZE,
        PteFlags::R | PteFlags::W,
    )?;

    // 代表页权限自检：代码 R-X、只读数据 R--、数据/BSS R-W、UART R-W，且都不设 U。
    check_page(
        &page_table,
        stext as *const () as usize,
        true,
        false,
        true,
        "text",
    )?;
    check_page(
        &page_table,
        srodata as *const () as usize,
        true,
        false,
        false,
        "rodata",
    )?;
    check_page(
        &page_table,
        sdata as *const () as usize,
        true,
        true,
        false,
        "data",
    )?;
    check_page(
        &page_table,
        sbss as *const () as usize,
        true,
        true,
        false,
        "bss",
    )?;
    check_page(&page_table, UART_BASE, true, true, false, "uart")?;

    // 遍历全部普通映射，确认 W 与 X 从不同时出现。
    for (start, end, _) in kernel_sections() {
        check_no_write_execute(&page_table, start, end)?;
    }
    check_no_write_execute(&page_table, UART_BASE, UART_BASE + PAGE_SIZE)?;

    // 每次页表写必须对随后读到 token 的其它 hart 可见，再发布。
    unsafe {
        core::arch::asm!("fence");
    }
    let token = (8usize << 60) | page_table.root_ppn().0;
    KERNEL_TOKEN.store(token, Ordering::Release);
    // 页表必须终身有效：先发布再 forget，否则 root 与中间页会被 Drop 回空闲链表。
    core::mem::forget(page_table);
    Ok(())
}

/// 遍历 `[start, end)` 的每个叶 PTE，若同时具有 W 与 X 则失败。
/// 注意用的是 `intersects`（有交集）而不是 `contains`（全含），二者语义不同。
fn check_no_write_execute(
    page_table: &PageTable,
    start: usize,
    end: usize,
) -> Result<(), &'static str> {
    let first = VirtAddr(start).floor().0;
    let last = VirtAddr(end).floor().0;
    for vpn in first..last {
        let entry = page_table
            .translate(VirtPageNum(vpn))
            .ok_or("kernel-space init: kernel page is not mapped")?;
        // 用“有交集”判定，避免 contains(R|W|X) 那种全含语义的陷阱。
        if entry.flags().intersects(PteFlags::W) && entry.flags().intersects(PteFlags::X) {
            return Err("kernel-space init: kernel page is both writable and executable");
        }
    }
    Ok(())
}

pub fn kernel_token() -> usize {
    KERNEL_TOKEN.load(Ordering::Acquire)
}
