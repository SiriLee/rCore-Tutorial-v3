#![no_std]
#![no_main]

use core::arch::asm;
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
        asm!("mret", options(noreturn));
    }
}

#[no_mangle]
extern "C" fn rust_main() -> ! {
    let hart_id = arch::hart_id();

    // 阶段一：UART 必须先就绪，之后任何 hart 才允许输出。
    if hart_id == 0 {
        uart::init();
        STARTED.store(true, Ordering::Release);
    } else {
        while !STARTED.load(Ordering::Acquire) {
            core::hint::spin_loop();
        }
    }

    // 阶段二：只有 hart 0 初始化页帧分配器并建立内核页表。
    if hart_id == 0 {
        extern "C" {
            fn ekernel();
        }

        unsafe {
            mm::frame_allocator_init(
                mm::PhysAddr(ekernel as *const () as usize),
                mm::PhysAddr(0x8800_0000),
            );
        }

        if !mm::run_tests() {
            println!("hart 0: memory self-test failed, paging left disabled");
            loop {
                arch::wait_for_interrupt();
            }
        }
    }

    // 阶段三：两个 hart 都 Acquire 等待 hart 0 发布的同一个 token。
    let token = loop {
        let token = mm::kernel_token();
        if token != 0 {
            break token;
        }
        core::hint::spin_loop();
    };
    unsafe { arch::activate_page_table(token) };
    println!("hart {} paging enabled", hart_id);

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
