use derive_more::Display;

use crate::arch::{
    aarch64::cpu::registers::daif::Daifset,
    arch_impl::cpu::registers::daif::{Daif, Daifclr, Mask},
};

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

impl ExceptionStatus {
    pub fn read() -> Self {
        let daif = Daif::fetch();

        ExceptionStatus {
            debug: daif.read(Daif::Debug).variant(),
            s_error: daif.read(Daif::SError).variant(),
            irq: daif.read(Daif::IRQ).variant(),
            fiq: daif.read(Daif::FIQ).variant(),
        }
    }
}

#[no_mangle]
pub fn unmask_irq() {
    Daifclr::write(Daifclr::Irq)
}

#[no_mangle]
pub fn mask_irq() {
    Daifset::write(Daifset::Irq)
}

pub fn local_irq_save() -> u64 {
    Daif::new().get()
}

pub fn local_irq_restore(state: u64) {
    Daif::new().set(state);
}
