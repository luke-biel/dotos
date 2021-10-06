#![allow(dead_code)]

use bitaccess::{bitaccess, FieldAccess};
use derive_more::Display;

#[derive(Display, FieldAccess)]
#[field_access(u64)]
pub enum Mask {
    Masked = 1,
    Unmasked = 0,
}

#[bitaccess(
    base_type = u64,
    kind = read_write,
    read_via = r#"unsafe { asm!("mrs {}, daif", out(reg) value, options(nostack, nomem)); }"#,
    write_via = r#"unsafe { asm!("msr daif, {}", in(reg) value, options(nostack, nomem)) }"#
)]
pub enum Daif {
    #[bit(9)]
    #[variants(Mask)]
    Debug,
    #[bit(8)]
    #[variants(Mask)]
    SError,
    #[bit(7)]
    #[variants(Mask)]
    IRQ,
    #[bit(6)]
    #[variants(Mask)]
    FIQ,
}

pub enum Daifset {
    Debug,
    SError,
    Irq,
    Fiq,
}

pub enum Daifclr {
    Debug,
    SError,
    Irq,
    Fiq,
}

impl Daifset {
    pub fn write(val: Self) {
        match val {
            Daifset::Debug => unsafe {
                asm!("msr daifset, {n}", n = const 0b1000, options(nostack, nomem))
            },
            Daifset::SError => unsafe {
                asm!("msr daifset, {n}", n = const 0b0100, options(nostack, nomem))
            },
            Daifset::Irq => unsafe {
                asm!("msr daifset, {n}", n = const 0b0010, options(nostack, nomem))
            },
            Daifset::Fiq => unsafe {
                asm!("msr daifset, {n}", n = const 0b0001, options(nostack, nomem))
            },
        }
    }
}

impl Daifclr {
    pub fn write(val: Self) {
        match val {
            Daifclr::Debug => unsafe {
                asm!("msr daifclr, {n}", n = const 0b1000, options(nostack, nomem))
            },
            Daifclr::SError => unsafe {
                asm!("msr daifclr, {n}", n = const 0b0100, options(nostack, nomem))
            },
            Daifclr::Irq => unsafe {
                asm!("msr daifclr, {n}", n = const 0b0010, options(nostack, nomem))
            },
            Daifclr::Fiq => unsafe {
                asm!("msr daifclr, {n}", n = const 0b0001, options(nostack, nomem))
            },
        }
    }
}
