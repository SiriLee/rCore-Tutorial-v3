// Minimal Linux-ABI RISC-V program: uses only write(64) and exit(93).
// This is the G1 milestone for the "run Linux binaries on rCore" project.
//
// Build: run project/scripts/build-programs.sh
#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit(101)
}

/// Linux syscall write(fd, buf, len): a7 = 64, args in a0/a1/a2.
fn write(fd: usize, buf: &[u8]) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "ecall",
            in("a7") 64usize,
            inlateout("a0") fd => ret,
            in("a1") buf.as_ptr(),
            in("a2") buf.len(),
            options(nostack),
        );
    }
    ret
}

/// Linux syscall exit(code): a7 = 93.
fn exit(code: i32) -> ! {
    unsafe {
        asm!(
            "ecall",
            in("a7") 93usize,
            in("a0") code as usize,
            options(noreturn, nostack),
        );
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write(1, b"[P1] hello from a Linux-ABI RISC-V binary\n");
    exit(0)
}
