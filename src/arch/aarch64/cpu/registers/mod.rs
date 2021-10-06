use crate::arch::{
    aarch64::cpu::registers::mpidr_el1::MpidrEl1,
    arch_impl::cpu::registers::current_el::{CurrentEl, ExceptionLevel},
};

pub mod current_el;
pub mod daif;
pub mod esr_el1;
pub mod far_el1;
pub mod mpidr_el1;
pub mod tcr_el1;

pub unsafe fn current_el() -> ExceptionLevel {
    CurrentEl::new().read(CurrentEl::Status).variant()
}

pub unsafe fn core_id_el1() -> u64 {
    MpidrEl1::new().read(MpidrEl1::CoreId).value()
}
