pub mod bcm;

use crate::bsp::raspberry_pi_3::mem::{GPIO_START, UART_START};
use core::marker::PhantomData;
use core::ops::Deref;

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

unsafe impl<T> Send for WrappedPointer<T> {}
unsafe impl<T> Sync for WrappedPointer<T> {}

impl<T> Deref for WrappedPointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.pointer as *const _) }
    }
}

pub static GPIO: bcm::gpio::Gpio = unsafe { bcm::gpio::Gpio::new(GPIO_START) };
pub static PL011_UART: bcm::pl011_uart::Uart = unsafe { bcm::pl011_uart::Uart::new(UART_START) };
