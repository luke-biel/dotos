use core::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub mod bcm;

pub struct WrappedPointer<T> {
    pointer: usize,
    _phantom: PhantomData<T>,
}

impl<T> WrappedPointer<T> {
    pub const unsafe fn new(pointer: usize) -> Self {
        Self {
            pointer,
            _phantom: PhantomData,
        }
    }
}

impl<T> fmt::Debug for WrappedPointer<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl<T> Deref for WrappedPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.pointer as *const _) }
    }
}

impl<T> DerefMut for WrappedPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.pointer as *mut _) }
    }
}
