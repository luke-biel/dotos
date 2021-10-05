use derive_more::Display;

use crate::arch::arch_impl::cpu::registers::daif::{Daif, Mask};

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

#[no_mangle]
pub fn disable_irq() {
    local_irq_set_mask(false);
}

pub fn local_irq_save() -> u64 {
    Daif::new().get()
}

pub fn local_irq_restore(state: u64) {
    Daif::new().set(state);
}

pub fn get_mask_state() -> ExceptionStatus {
    let daif = Daif::fetch();

    ExceptionStatus {
        debug: daif.read(Daif::Debug).variant(),
        s_error: daif.read(Daif::SError).variant(),
        irq: daif.read(Daif::IRQ).variant(),
        fiq: daif.read(Daif::FIQ).variant(),
    }
}
