use core::ptr::{read_volatile, write_volatile};

const PLIC: usize = 0x0c00_0000;
pub const UART_IRQ: u32 = 10;
const S_ENABLE_HART0: usize = PLIC + 0x2080;
const S_THRESHOLD_HART0: usize = PLIC + 0x201000;
const S_CLAIM_HART0: usize = PLIC + 0x201004;

pub fn init() {
    unsafe {
        write_volatile((PLIC + UART_IRQ as usize * 4) as *mut u32, 1);
    }
}

pub fn init_hart(hart_id: usize) {
    if hart_id != 0 {
        return;
    }
    unsafe {
        write_volatile(S_ENABLE_HART0 as *mut u32, 1u32 << UART_IRQ);
        write_volatile(S_THRESHOLD_HART0 as *mut u32, 0);
    }
}

pub fn claim() -> u32 {
    unsafe { read_volatile(S_CLAIM_HART0 as *const u32) }
}

pub fn complete(irq: u32) {
    if irq == 0 {
        return;
    }
    unsafe {
        write_volatile(S_CLAIM_HART0 as *mut u32, irq);
    }
}
