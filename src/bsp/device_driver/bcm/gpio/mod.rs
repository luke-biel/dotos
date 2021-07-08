use crate::common::driver::DeviceDriver;
use crate::common::sync::{Mutex, NullLock};
use inner::GpioInner;

mod inner;
mod registers;

pub struct Gpio {
    inner: NullLock<GpioInner>,
}

impl Gpio {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: NullLock::new(GpioInner::new(mmio_start_addr)),
        }
    }

    pub fn map_pl011_uart(&self) {
        self.inner.map_locked(|gpio| gpio.map_pl011_uart());
    }
}

impl DeviceDriver for Gpio {
    fn compat(&self) -> &'static str {
        "BCM GPIO"
    }

    unsafe fn late_init(&self) -> Result<(), &'static str> {
        self.map_pl011_uart();

        Ok(())
    }
}
