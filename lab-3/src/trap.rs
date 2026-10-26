use crate::{arch, plic, uart};

const SCAUSE_INTERRUPT: usize = 1usize << (usize::BITS - 1);
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
    plic::init();
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
    let _ = context;
    let scause = arch::read_scause();

    if scause & SCAUSE_INTERRUPT != 0 && scause & !SCAUSE_INTERRUPT == S_EXTERNAL {
        handle_external_interrupt();
        return;
    }

    report_unknown_trap(scause);
}

fn handle_external_interrupt() {
    let irq = plic::claim();
    if irq == plic::UART_IRQ {
        uart::handle_interrupt();
    } else if irq != 0 {
        println!("lab-3 unknown external interrupt: irq={}", irq);
    }

    // 每个非零 claim 必须在同 context 恰好 complete 一次。
    if irq != 0 {
        plic::complete(irq);
    }
}

fn report_unknown_trap(scause: usize) -> ! {
    println!(
        "lab-3 unknown trap: scause={:#x} sepc={:#x} stval={:#x}",
        scause,
        arch::read_sepc(),
        arch::read_stval()
    );
    loop {
        arch::wait_for_interrupt();
    }
}
