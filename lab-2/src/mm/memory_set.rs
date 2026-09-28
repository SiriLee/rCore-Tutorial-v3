use core::sync::atomic::{AtomicUsize, Ordering};

use super::{PageTable, PhysAddr, PteFlags, VirtAddr, PAGE_SIZE};

const MEMORY_END: usize = 0x8800_0000;
const UART_BASE: usize = 0x1000_0000;
static KERNEL_TOKEN: AtomicUsize = AtomicUsize::new(0);

pub fn kernel_space_init() -> Result<(), &'static str> {
    let _ = (MEMORY_END, UART_BASE, PAGE_SIZE, PhysAddr(0), VirtAddr(0));
    let _ = (
        PteFlags::R,
        PteFlags::W,
        PteFlags::X,
        PteFlags::U,
        PageTable::new,
    );
    // TODO(task 6): hart 0 建立内核恒等映射，将 satp token 以 Release 发布。
    Err("kernel address space is not implemented")
}

pub fn kernel_token() -> usize {
    KERNEL_TOKEN.load(Ordering::Acquire)
}
