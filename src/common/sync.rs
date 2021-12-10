use core::cell::UnsafeCell;

use bitaccess::ReadBits;

use crate::{
    arch::arch_impl::cpu::{
        exception::asynchronous::{local_irq_restore, local_irq_save, mask_irq},
        registers::daif::Daif,
    },
    common::statics,
};

pub trait Mutex {
    type Data;
    fn map_locked<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&mut Self::Data) -> R;
}

pub trait ReadWriteLock {
    type Data;
    fn map_read<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&Self::Data) -> R;
    fn map_write<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&mut Self::Data) -> R;
}

/// Single threaded synchronization provider.
/// Don't use on m-t environments
pub struct IRQSafeNullLock<T: Sized> {
    data: UnsafeCell<T>,
}

unsafe impl<T> Send for IRQSafeNullLock<T> where T: Send {}
unsafe impl<T> Sync for IRQSafeNullLock<T> where T: Send {}

impl<T: Sized> IRQSafeNullLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

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

pub struct InitStateLock<T: ?Sized> {
    data: UnsafeCell<T>,
}

impl<T> InitStateLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

unsafe impl<T> Send for InitStateLock<T> where T: Send {}
unsafe impl<T> Sync for InitStateLock<T> where T: Send {}

impl<T> ReadWriteLock for InitStateLock<T> {
    type Data = T;

    fn map_read<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&Self::Data) -> R,
    {
        let data = unsafe { &*self.data.get() };
        f(data)
    }

    // No additional synchronization required, as this lock allows writes on single core with IRQs masked
    fn map_write<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&mut Self::Data) -> R,
    {
        if !statics::STATE_MANAGER.is_init() {
            panic!("Called InitStateLock after init")
        }
        if Daif.read(Daif::IRQ).value() == 0 {
            panic!("Called InitStateLock with IRQ unmasked")
        }

        let data = unsafe { &mut *self.data.get() };

        f(data)
    }
}
