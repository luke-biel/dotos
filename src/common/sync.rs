use core::cell::UnsafeCell;

pub trait Mutex {
    type Data;
    fn map_locked<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&mut Self::Data) -> R;
}

/// Single threaded synchronization provider.
/// Don't use on m-t environments or with interrupts turned on.
pub struct NullLock<T: Sized> {
    data: UnsafeCell<T>,
}

unsafe impl<T> Send for NullLock<T> where T: Send {}
unsafe impl<T> Sync for NullLock<T> where T: Send {}

impl<T: Sized> NullLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

impl<T: Sized> Mutex for NullLock<T> {
    type Data = T;

    fn map_locked<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&mut Self::Data) -> R,
    {
        let data = unsafe { &mut *self.data.get() };

        f(data)
    }
}
