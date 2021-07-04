use crate::common::driver::DeviceDriver;
use crate::common::sync::{NullLock, Mutex};
use crate::bsp::device_driver::bcm::pl011_uart::inner::UartInner;

mod inner;
mod registers;

pub struct Uart {
    inner: NullLock<UartInner>,
}

impl Uart {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: NullLock::new(UartInner::new(mmio_start_addr)),
        }
    }

    pub fn map_pl011_uart(&self) {
        self.inner.map_locked(|uart| uart.map_pl011_uart());
    }

    pub fn write_blocking(&self, s: &str) {
        self.inner.map_locked(|uart| uart.write_blocking(s))
    }

    pub fn read_char_blocking(&self) -> u8 {
        self.inner.map_locked(|uart| uart.read_char_blocking())
    }
}

impl DeviceDriver for Uart {
    fn compat(&self) -> &'static str {
        "BCM PL011 UART"
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        self.map_pl011_uart();

        Ok(())
    }
}
