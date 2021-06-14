#![allow(dead_code)]

use core::cell::UnsafeCell;

use crate::arch::IntPtr;
use crate::pointer_iter::PointerIter;

extern "Rust" {
    static __bss_start: UnsafeCell<IntPtr>;
    static __bss_end: UnsafeCell<IntPtr>;
}

// pub const MMIO_BASE: u32 = 0x2000_0000;
//
// pub const GPIO_BASE: u32 = MMIO_BASE + 0x20_0000;
//
// pub const GPPUD: u32 = GPIO_BASE + 0x94;
//
// pub const GPPUDCLK0: u32 = GPIO_BASE + 0x98;
//
// pub const UART0_BASE: u32 = GPIO_BASE + 0x1000; // 0x2020_1000
//
// pub const UART0_DR: u32 = UART0_BASE;
// pub const UART0_RSRECR: u32 = UART0_BASE + 0x04;
// pub const UART0_FR: u32 = UART0_BASE + 0x18;
// pub const UART0_ILPR: u32 = UART0_BASE + 0x20;
// pub const UART0_IBRD: u32 = UART0_BASE + 0x24;
// pub const UART0_FBRD: u32 = UART0_BASE + 0x28;
// pub const UART0_LCRH: u32 = UART0_BASE + 0x2C;
// pub const UART0_CR: u32 = UART0_BASE + 0x30;
// pub const UART0_IFLS: u32 = UART0_BASE + 0x34;
// pub const UART0_IMSC: u32 = UART0_BASE + 0x38;
// pub const UART0_RIS: u32 = UART0_BASE + 0x3C;
// pub const UART0_MIS: u32 = UART0_BASE + 0x40;
// pub const UART0_ICR: u32 = UART0_BASE + 0x44;
// pub const UART0_DMACR: u32 = UART0_BASE + 0x48;
// pub const UART0_ITCR: u32 = UART0_BASE + 0x80;
// pub const UART0_ITIP: u32 = UART0_BASE + 0x84;
// pub const UART0_ITOP: u32 = UART0_BASE + 0x88;
// pub const UART0_TDR: u32 = UART0_BASE + 0x8C;
//
// pub const MBOX_BASE: u32 = MMIO_BASE + 0xB880;
// pub const MBOX_READ: u32 = MBOX_BASE + 0x00;
// pub const MBOX_STATUS: u32 = MBOX_BASE + 0x18;
// pub const MBOX_WRITE: u32 = MBOX_BASE + 0x20;

pub fn bss_section() -> PointerIter<IntPtr> {
    unsafe { PointerIter::new(__bss_start.get(), __bss_end.get()) }
}
