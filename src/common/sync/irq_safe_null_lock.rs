use core::cell::UnsafeCell;

use crate::arch::aarch64::cpu::exception::asynchronous::{
    local_irq_restore,
    local_irq_save,
    mask_irq,
};

pub trait Mutex {
    type Data;
    fn map_locked<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&mut Self::Data) -> R;
}

pub struct IRQSafeNullLock<T: Sized> {
    data: UnsafeCell<T>,
}

impl<T: Sized> IRQSafeNullLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

unsafe impl<T> Send for IRQSafeNullLock<T> where T: Send {}
unsafe impl<T> Sync for IRQSafeNullLock<T> where T: Send {}

impl<T: Sized> Mutex for IRQSafeNullLock<T> {
    type Data = T;

    fn map_locked<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&mut Self::Data) -> R,
    {
        let data = unsafe { &mut *self.data.get() };

        let state = local_irq_save();
        mask_irq();
        let res = f(data);
        local_irq_restore(state);

        res
    }
}
