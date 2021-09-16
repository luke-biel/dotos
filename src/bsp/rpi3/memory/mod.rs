use core::{cell::UnsafeCell, ops::Range};

use crate::common::memory::{Address, Virtual};

pub mod map;
pub mod mmu;

extern "Rust" {
    static __bss_start: UnsafeCell<()>;
    static __bss_ende: UnsafeCell<()>;

    static __rx_start: UnsafeCell<()>;
    static __rx_ende: UnsafeCell<()>;

    static __rw_start: UnsafeCell<()>;
    static __rw_ende: UnsafeCell<()>;

    static __boot_core_stack_start: UnsafeCell<()>;
    static __boot_core_stack_ende: UnsafeCell<()>;
}

fn bss_start() -> usize {
    unsafe { __bss_start.get() as usize }
}

fn bss_ende() -> usize {
    unsafe { __bss_ende.get() as usize }
}

pub fn bss() -> Range<usize> {
    bss_start()..bss_ende()
}

pub fn rx_start() -> Address<Virtual> {
    Address::new(unsafe { __rx_start.get() as usize })
}

pub fn rx_size() -> usize {
    unsafe { (__rx_ende.get() as usize) - (__rx_start.get() as usize) }
}

pub fn rw_start() -> Address<Virtual> {
    Address::new(unsafe { __rw_start.get() as usize })
}

pub fn rw_size() -> usize {
    unsafe { (__rw_ende.get() as usize) - (__rw_start.get() as usize) }
}

pub fn boot_core_stack_start() -> Address<Virtual> {
    Address::new(unsafe { __boot_core_stack_start.get() as usize })
}

pub fn boot_core_stack_ende() -> Address<Virtual> {
    Address::new(unsafe { __boot_core_stack_ende.get() as usize })
}

pub fn boot_core_stack_size() -> usize {
    unsafe { (__boot_core_stack_ende.get() as usize) - (__boot_core_stack_start.get() as usize) }
}
