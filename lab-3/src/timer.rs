use core::arch::asm;
use core::ptr::addr_of_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::arch;

pub const INTERVAL: usize = 1_000_000;
const CLINT_MTIME: usize = 0x0200_bff8;
const CLINT_MTIMECMP: usize = 0x0200_4000;
const MAX_HARTS: usize = 2;

static TICKS: AtomicUsize = AtomicUsize::new(0);
static mut TIMER_SCRATCH: [[usize; 5]; MAX_HARTS] = [[0; 5]; MAX_HARTS];

unsafe extern "C" {
    fn timer_vector();
}

/// # Safety
/// 只能由对应 hart 在 M-mode 启动阶段调用一次。
pub unsafe fn init_machine(hart_id: usize) {
    assert!(hart_id < MAX_HARTS);

    let mtimecmp = (CLINT_MTIMECMP + hart_id * 8) as *mut usize;
    let now = core::ptr::read_volatile(CLINT_MTIME as *const usize);
    core::ptr::write_volatile(mtimecmp, now.wrapping_add(INTERVAL));

    // 每个 hart 独占五字 scratch：[a1, a2, a3, mtimecmp, interval]。
    let base = addr_of_mut!(TIMER_SCRATCH).cast::<usize>();
    let scratch = base.add(hart_id * 5);
    scratch.add(3).write(mtimecmp as usize);
    scratch.add(4).write(INTERVAL);

    asm!("csrw mscratch, {}", in(reg) scratch);
    asm!("csrw mtvec, {}", in(reg) timer_vector as *const () as usize);
    asm!("csrs mstatus, {}", in(reg) 1usize << 3);
    asm!("csrs mie, {}", in(reg) 1usize << 7);
}

pub fn update(hart_id: usize) {
    if hart_id == 0 {
        TICKS.fetch_add(1, Ordering::AcqRel);
    }
    // 两个 hart 都要清自己的 SSIP，否则会反复进入同一个软件中断。
    arch::clear_supervisor_software_interrupt();
}

pub fn ticks() -> usize {
    TICKS.load(Ordering::Acquire)
}
