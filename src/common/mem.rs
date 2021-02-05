use crate::arch::IntPtr;
use crate::pointer_iter::PointerIter;
use core::ptr::write_volatile;

pub unsafe fn clear_region(range: PointerIter<IntPtr>) {
    for ptr in range {
        write_volatile(ptr, 0)
    }
}
