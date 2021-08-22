use core::{
    fmt,
    fmt::{Display, Formatter},
};

bitflags::bitflags! {
#[repr(C)]
struct Daif: u64 {
    const DEBUG = 1 << 9;
    const SERROR = 1 << 8;
    const IRQ = 1 << 7;
    const FIQ = 1 << 6;
}
}

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

impl Display for Mask {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Mask::Masked => "masked",
            Mask::Unmasked => "unmasked",
        }
            .fmt(f)
    }
}

pub struct ExceptionStatus {
    pub debug: Mask,
    pub s_error: Mask,
    pub irq: Mask,
    pub fiq: Mask,
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

impl fmt::Display for ExceptionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Debug: {}, SError: {}, IRQ: {}, FIQ: {}",
            self.debug, self.s_error, self.irq, self.fiq
        )
    }
}
