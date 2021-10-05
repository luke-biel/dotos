use crate::arch::arch_impl::cpu::registers::current_el::{CurrentEl, ExceptionLevel};

pub mod current_el;
pub mod esr_el1;
pub mod tcr_el1;

const CORE_ID_MASK: usize = 0b11;

pub unsafe fn current_el() -> ExceptionLevel {
    CurrentEl::new().read(CurrentEl::Status).variant()
}

pub unsafe fn core_id_el1() -> usize {
    let v: usize;
    asm!("mrs {}, mpidr_el1", out(reg) v, options(nomem, nostack));

    v & CORE_ID_MASK
}
