use core::arch::asm;

pub const MSTATUS_MPP_MASK: usize = 3 << 11;
pub const MSTATUS_MPP_S: usize = 1 << 11;
pub const PMP_R: usize = 1 << 0;
pub const PMP_W: usize = 1 << 1;
pub const PMP_X: usize = 1 << 2;
pub const PMP_A_TOR: usize = 1 << 3;
const SSTATUS_SIE: usize = 1 << 1;

#[inline]
pub fn read_mstatus() -> usize {
    let value;
    unsafe { asm!("csrr {}, mstatus", out(reg) value) };
    value
}

#[inline]
pub unsafe fn write_mstatus(value: usize) {
    asm!("csrw mstatus, {}", in(reg) value);
}

#[inline]
pub unsafe fn write_mepc(value: usize) {
    asm!("csrw mepc, {}", in(reg) value);
}

#[inline]
pub unsafe fn write_satp(value: usize) {
    asm!("csrw satp, {}", in(reg) value);
}

#[inline]
pub unsafe fn write_pmpaddr0(value: usize) {
    asm!("csrw pmpaddr0, {}", in(reg) value);
}

#[inline]
pub unsafe fn write_pmpcfg0(value: usize) {
    asm!("csrw pmpcfg0, {}", in(reg) value);
}

#[inline]
pub unsafe fn write_tp(value: usize) {
    asm!("mv tp, {}", in(reg) value);
}

#[inline]
pub fn hart_id() -> usize {
    let value;
    unsafe { asm!("mv {}, tp", out(reg) value) };
    value
}

#[inline]
fn read_sstatus() -> usize {
    let value;
    unsafe { asm!("csrr {}, sstatus", out(reg) value) };
    value
}

#[inline]
pub fn interrupts_enabled() -> bool {
    read_sstatus() & SSTATUS_SIE != 0
}

#[inline]
pub fn enable_interrupts() {
    unsafe { asm!("csrs sstatus, {}", in(reg) SSTATUS_SIE) };
}

#[inline]
pub fn disable_interrupts() {
    unsafe { asm!("csrc sstatus, {}", in(reg) SSTATUS_SIE) };
}

#[inline]
pub fn wait_for_interrupt() {
    unsafe { asm!("wfi") };
}
