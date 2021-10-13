use bitaccess::ReadBits;

use crate::arch::aarch64::cpu::registers::mpidr_el1::MpidrEl1;

pub mod cnthctl_el2;
pub mod cntvoff_el2;
pub mod current_el;
pub mod daif;
pub mod elr_el2;
pub mod esr_el1;
pub mod far_el1;
pub mod hcr_el2;
pub mod mpidr_el1;
pub mod sp_el1;
pub mod spsr_el2;
pub mod tcr_el1;

pub unsafe fn core_id_el1() -> u64 {
    MpidrEl1::new().read(MpidrEl1::CoreId).value()
}
