use core::arch::asm;

pub const MSTATUS_MPP_MASK: usize = 3 << 11;
pub const MSTATUS_MPP_S: usize = 1 << 11;
pub const PMP_R: usize = 1 << 0;
pub const PMP_W: usize = 1 << 1;
pub const PMP_X: usize = 1 << 2;
pub const PMP_A_TOR: usize = 1 << 3;
const SSTATUS_SIE: usize = 1 << 1;
const SIE_SSIE: usize = 1 << 1;
const SIE_SEIE: usize = 1 << 9;

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
pub unsafe fn write_medeleg(value: usize) {
    asm!("csrw medeleg, {}", in(reg) value);
}

#[inline]
pub unsafe fn write_mideleg(value: usize) {
    asm!("csrw mideleg, {}", in(reg) value);
}

#[inline]
pub unsafe fn set_stvec(value: usize) {
    asm!("csrw stvec, {}", in(reg) value);
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
pub fn read_scause() -> usize {
    let value;
    unsafe { asm!("csrr {}, scause", out(reg) value) };
    value
}

#[inline]
pub fn read_sepc() -> usize {
    let value;
    unsafe { asm!("csrr {}, sepc", out(reg) value) };
    value
}

#[inline]
pub fn read_stval() -> usize {
    let value;
    unsafe { asm!("csrr {}, stval", out(reg) value) };
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

/// 打开 S-mode 软件与外部中断，最后再打开 S-mode 总开关。
/// 初始化期间不能提前打开 STIE：本实验尚未处理 S-mode 时钟中断。
#[inline]
pub fn enable_supervisor_trap_interrupts() {
    unsafe {
        asm!("csrs sie, {}", in(reg) SIE_SSIE | SIE_SEIE);
        asm!("csrs sstatus, {}", in(reg) SSTATUS_SIE);
    }
}

#[inline]
pub fn wait_for_interrupt() {
    unsafe { asm!("wfi") };
}

#[inline]
pub unsafe fn activate_page_table(token: usize) {
    asm!("csrw satp, {}", in(reg) token);
    asm!("sfence.vma");
}
