use core::cell::UnsafeCell;
use crate::arch::IntPtr;
use crate::pointer_iter::PointerIter;

extern "Rust" {
    static __bss_start: UnsafeCell<IntPtr>;
    static __bss_end: UnsafeCell<IntPtr>;
}

pub fn bss_section() -> PointerIter<IntPtr> {
    unsafe { PointerIter::new(__bss_start.get(), __bss_end.get()) }
}
