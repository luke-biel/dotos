use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub mod esr_el1;

const CORE_ID_MASK: usize = 0b11;

#[derive(PartialEq, FromPrimitive)]
pub enum ExceptionLevel {
    EL0 = 0b0000,
    EL1 = 0b0100,
    EL2 = 0b1000,
    EL3 = 0b1100,
}

pub unsafe fn current_el() -> ExceptionLevel {
    let v: usize;
    asm!("mrs {}, currentel", out(reg) v, options(nomem, nostack));

    ExceptionLevel::from_usize(v).unwrap_or(ExceptionLevel::EL0)
}

pub unsafe fn core_id_el1() -> usize {
    let v: usize;
    asm!("mrs {}, mpidr_el1", out(reg) v, options(nomem, nostack));

    v & CORE_ID_MASK
}
