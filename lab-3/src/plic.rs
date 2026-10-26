use core::ptr::{read_volatile, write_volatile};

const PLIC: usize = 0x0c00_0000;
pub const UART_IRQ: u32 = 10;
const S_ENABLE_HART0: usize = PLIC + 0x2080;
const S_THRESHOLD_HART0: usize = PLIC + 0x201000;
const S_CLAIM_HART0: usize = PLIC + 0x201004;

pub fn init() {
    // TODO(task 5): 把 UART IRQ 10 的优先级设为 1。
    let _ = write_volatile as unsafe fn(*mut u32, u32);
}

pub fn init_hart(hart_id: usize) {
    // TODO(task 5): 本实验只让 hart 0 接收 UART 外部中断。
    let _ = (hart_id, S_ENABLE_HART0, S_THRESHOLD_HART0);
}

pub fn claim() -> u32 {
    // TODO(task 5): 读取 hart 0 S-context 的 claim/complete 寄存器。
    let _ = read_volatile as unsafe fn(*const u32) -> u32;
    let _ = S_CLAIM_HART0;
    0
}

pub fn complete(irq: u32) {
    // TODO(task 5): 把 claim 得到的非零 IRQ 写回同一寄存器。
    let _ = irq;
}
