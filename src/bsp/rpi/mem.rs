#![allow(dead_code)]

use core::cell::UnsafeCell;

use crate::arch::IntPtr;
use crate::pointer_iter::PointerIter;

extern "Rust" {
    static __bss_start: UnsafeCell<IntPtr>;
    static __bss_end: UnsafeCell<IntPtr>;
}

pub const GPIO_OFFSET: usize = 0x20_0000;
pub const UART_OFFSET: usize = 0x20_1000;

cfg_if::cfg_if! {
    if #[cfg(feature = "rpi1")] {
        const MMIO_BASE: usize = 0x2000_0000;
    }
}

pub const GPIO_START: usize = MMIO_BASE + GPIO_OFFSET;
pub const UART_START: usize = MMIO_BASE + UART_OFFSET;

pub fn bss_section() -> PointerIter<IntPtr> {
    unsafe { PointerIter::new(__bss_start.get(), __bss_end.get()) }
}
