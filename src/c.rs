use core::cmp::Ordering;

use crate::arch::{Int, IntPtr, UInt};

#[no_mangle]
unsafe fn memcmp(ptr1: IntPtr, ptr2: IntPtr, num: UInt) -> Int {
    for i in 0..num {
        let left = ptr1 as *const u8;
        let right = ptr2 as *const u8;

        match (*left.offset(i as isize)).cmp(&*right.offset(i as isize)) {
            Ordering::Less => return -1,
            Ordering::Equal => (),
            Ordering::Greater => return 1,
        }
    }

    0
}

#[no_mangle]
// TODO: Review function, it's not really sync and when I'll switch to multithreading, this may be problematic
/// Spec: https://gcc.gnu.org/onlinedocs/gcc/_005f_005fsync-Builtins.html
unsafe fn __sync_val_compare_and_swap_1(ptr: *mut u8, old_val: u8, new_val: u8) -> u8 {
    if *ptr == old_val {
        ptr.write(new_val);
        return old_val
    }
    *ptr
}

#[no_mangle]
// TODO: Review function, it's not really sync and when I'll switch to multithreading, this may be problematic
/// Spec: https://gcc.gnu.org/onlinedocs/gcc/_005f_005fsync-Builtins.html
unsafe fn __sync_lock_test_and_set_1(ptr: *mut u8, new_val: u8) -> u8 {
    let temp = *ptr;
    ptr.write(new_val);
    temp
}
