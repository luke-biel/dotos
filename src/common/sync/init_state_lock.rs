use core::cell::UnsafeCell;

use bitaccess::ReadBits;

use crate::{arch::aarch64::cpu::registers::daif::Daif, statics};

pub trait ReadWriteLock {
    type Data;
    fn map_read<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&Self::Data) -> R;
    fn map_write<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&mut Self::Data) -> R;
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
