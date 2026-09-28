#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};

static STARTED: AtomicBool = AtomicBool::new(false);

mod arch;
#[macro_use]
mod console;
mod mm;
mod sync;
mod uart;

core::arch::global_asm!(include_str!("entry.S"));

#[no_mangle]
extern "C" fn start(hart_id: usize) -> ! {
    let mut mstatus = arch::read_mstatus();
    mstatus &= !arch::MSTATUS_MPP_MASK;
    mstatus |= arch::MSTATUS_MPP_S;

    unsafe {
        arch::write_mstatus(mstatus);
        arch::write_mepc(rust_main as *const () as usize);
        arch::write_satp(0);
        arch::write_pmpaddr0(usize::MAX);
        arch::write_pmpcfg0(arch::PMP_R | arch::PMP_W | arch::PMP_X | arch::PMP_A_TOR);
        arch::write_tp(hart_id);
        core::arch::asm!("mret", options(noreturn));
    }
}

#[no_mangle]
extern "C" fn rust_main() -> ! {
    let hart_id = arch::hart_id();

    if hart_id == 0 {
        uart::init();
        println!("ECNU OSLab rCore kernel entered S-mode");
        if sync::interrupt_nesting_selftest() {
            println!("interrupt nesting self-test: PASS");
        } else {
            println!("interrupt nesting self-test: FAIL");
        }
        println!(
            "Rust formatting: char={} string={} d={} p={:x} x={:#x}",
            'A', "uart", -2025, 0x2025_u32, 0x1234_5678_8000_0000_u64,
        );
        STARTED.store(true, Ordering::Release);
    } else {
        while !STARTED.load(Ordering::Acquire) {
            core::hint::spin_loop();
        }
    }

    let tag = if hart_id == 0 { 'A' } else { 'B' };
    println!(
        "hart {} says: char={} string={} d={} p={:x} x={:#x}",
        hart_id, tag, "uart", -2025, 0x2025_u32, 0x1234_5678_8000_0000_u64,
    );

    loop {
        arch::wait_for_interrupt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        arch::wait_for_interrupt();
    }
}
