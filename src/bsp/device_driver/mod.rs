use core::{marker::PhantomData, ops::Deref};

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

    pub fn addr(&self) -> usize {
        self.pointer
    }
}

impl<T> Deref for WrappedPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.pointer as *const _) }
    }
}
