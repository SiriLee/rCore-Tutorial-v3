use crate::{arch, plic, timer, uart};

const SCAUSE_INTERRUPT: usize = 1usize << (usize::BITS - 1);
const S_SOFTWARE: usize = 1;
const S_EXTERNAL: usize = 9;

unsafe extern "C" {
    fn kernel_vector();
}

#[repr(C)]
pub struct KernelTrapContext {
    pub x: [usize; 32],
}

pub fn context_layout_ok() -> bool {
    core::mem::size_of::<KernelTrapContext>() == 32 * core::mem::size_of::<usize>()
}

pub fn init_global() {
    // TODO(task 5): 初始化 PLIC 全局优先级。
}

pub fn init_hart() {
    unsafe {
        arch::set_stvec(kernel_vector as *const () as usize);
    }
    plic::init_hart(arch::hart_id());
    // 总开关必须是最后一步：入口和 PLIC 都就绪后才允许中断到达。
    arch::enable_supervisor_trap_interrupts();
}

#[no_mangle]
extern "C" fn kernel_trap_handler(context: &mut KernelTrapContext) {
    // TODO(task 6): 保存 CSR 快照，精确分派 SSIP 和 SEIP；未知 trap 停机。
    let _ = (
        context,
        SCAUSE_INTERRUPT,
        S_SOFTWARE,
        S_EXTERNAL,
        arch::hart_id(),
        plic::UART_IRQ,
        timer::ticks(),
        uart::putchar as fn(u8),
    );
}
