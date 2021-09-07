use core::{
    fmt,
    fmt::{Display, Formatter},
};

use derive_more::Display;

bitflags::bitflags! {
    #[repr(C)]
    pub struct Daif: u64 {
        const DEBUG = 1 << 9;
        const SERROR = 1 << 8;
        const IRQ = 1 << 7;
        const FIQ = 1 << 6;
    }
}

#[derive(Display)]
pub enum Mask {
    Masked,
    Unmasked,
}

impl From<bool> for Mask {
    fn from(v: bool) -> Self {
        if v {
            Mask::Masked
        } else {
            Mask::Unmasked
        }
    }
}

#[derive(Display)]
#[display(
    fmt = "debug = {}; serror = {}; irq = {}; fiq = {};",
    debug,
    s_error,
    irq,
    fiq
)]
pub struct ExceptionStatus {
    pub debug: Mask,
    pub s_error: Mask,
    pub irq: Mask,
    pub fiq: Mask,
}

pub fn local_irq_set_mask(mask: bool) {
    const IRQ: u8 = 0b0010;
    unsafe {
        if mask {
            asm!("msr daifset, {arg}", arg = const IRQ, options(nostack, nomem, preserves_flags));
        } else {
            asm!("msr daifclr, {arg}", arg = const IRQ, options(nostack, nomem, preserves_flags));
        }
    }
}

pub fn local_irq_save() -> u64 {
    let daif: u64;
    unsafe { asm!("mrs {}, daif", out(reg) daif, options(nostack, nomem)) };
    daif
}

pub fn local_irq_restore(state: u64) {
    unsafe { asm!("msr daif, {}", in(reg) state, options(nostack, nomem)) };
}

pub fn get_mask_state() -> ExceptionStatus {
    let daif: u64;
    unsafe { asm!("mrs {}, daif", out(reg) daif, options(nostack, nomem)) };
    let daif = Daif::from_bits(daif).unwrap();

    ExceptionStatus {
        debug: daif.contains(Daif::DEBUG).into(),
        s_error: daif.contains(Daif::SERROR).into(),
        irq: daif.contains(Daif::IRQ).into(),
        fiq: daif.contains(Daif::FIQ).into(),
    }
}

impl Daif {
    pub fn state() -> Self {
        let daif: u64;
        unsafe { asm!("mrs {}, daif", out(reg) daif, options(nostack, nomem)) };
        Daif::from_bits(daif).unwrap()
    }
}
