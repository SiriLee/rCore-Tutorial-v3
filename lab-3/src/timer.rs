use core::sync::atomic::{AtomicUsize, Ordering};

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
    // TODO(task 4): 设置 mtimecmp、mscratch、mtvec、mstatus.MIE 和 mie.MTIE。
    let _ = (
        hart_id,
        CLINT_MTIME,
        CLINT_MTIMECMP,
        INTERVAL,
        timer_vector as *const () as usize,
    );
}

pub fn update(hart_id: usize) {
    // TODO(task 4): 只由 hart 0 增加全局 ticks，并清除 sip.SSIP。
    let _ = (hart_id, &TICKS, Ordering::Relaxed);
}

pub fn ticks() -> usize {
    TICKS.load(Ordering::Acquire)
}
