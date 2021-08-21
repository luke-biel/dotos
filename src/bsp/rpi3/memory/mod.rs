use core::{cell::UnsafeCell, ops::Range};

pub mod map;
pub mod mmu;

extern "Rust" {
    static __boot_core_stack_ende: UnsafeCell<()>;

    static __bss_start: UnsafeCell<()>;
    static __bss_ende: UnsafeCell<()>;

    static __rx_start: UnsafeCell<()>;
    static __rx_ende: UnsafeCell<()>;
}

pub fn boot_core_stack_ende() -> usize {
    unsafe { __boot_core_stack_ende.get() as usize }
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

fn rx_start() -> usize {
    unsafe { __rx_start.get() as usize }
}

fn rx_ende() -> usize {
    unsafe { __rx_ende.get() as usize }
}
