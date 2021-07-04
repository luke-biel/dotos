use tock_registers::registers::{Readable, Writeable};

use crate::arch::cpu::nop;
use crate::bsp::device_driver::bcm::bcm2xxx_pl011_uart::{UARTRegisterBlock, CR, FR, LCRH};
use crate::bsp::device_driver::WrappedPointer;
use crate::common::driver::DeviceDriver;
use spin::Mutex;

struct UARTInner {
    block: WrappedPointer<UARTRegisterBlock>,
}

pub struct UART {
    inner: Mutex<UARTInner>,
}

impl UARTInner {
    const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            block: WrappedPointer::new(mmio_start_addr),
        }
    }

    fn write_blocking(&self, s: &str) {
        for c in s.chars() {
            while self.block.fr.matches_all(FR::TXFF::Full) {}
            self.block.dr.set(c as u32);
        }

        self.flush();
    }

    fn read_char(&self) -> Option<u8> {
        if self.block.fr.matches_all(FR::RXFE::Empty) {
            return None;
        }

        Some(self.block.dr.get() as u8)
    }

    fn read_char_blocking(&self) -> u8 {
        while self.block.fr.matches_all(FR::RXFE::Empty) {}

        let c = self.block.dr.get() as u8;

        if c == b'\r' {
            b'\n'
        } else {
            c
        }
    }

    fn clear_rx(&self) {
        while self.read_char().is_some() {}
    }

    fn flush(&self) {
        while self.block.fr.matches_all(FR::BUSY::SET + FR::TXFF::Full) {}
        crate::arch::cpu::sleep(24_000);
    }

    fn map_pl011_uart(&self) {
        self.flush();

        self.block.cr.set(0);

        self.block.icr.set(0x7FF);

        self.block.ibrd.set(1);
        self.block.fbrd.set(0x28);

        self.block.lcrh.write(LCRH::FEN::Enabled + LCRH::WLEN::Len8);

        self.block.imsc.set(0x7f1);

        self.block
            .cr
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);
    }
}

impl UART {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: spin::Mutex::new(UARTInner::new(mmio_start_addr)),
        }
    }

    pub fn map_pl011_uart(&self) {
        self.inner.lock().map_pl011_uart()
    }

    pub fn write_blocking(&self, s: &str) {
        self.inner.lock().write_blocking(s)
    }

    pub fn read_char(&self) -> Option<u8> {
        self.inner.lock().read_char()
    }

    pub fn read_char_blocking(&self) -> u8 {
        self.inner.lock().read_char_blocking()
    }

    pub fn clear_rx(&self) {
        self.inner.lock().clear_rx()
    }
}

impl DeviceDriver for UART {
    fn compat(&self) -> &'static str {
        "BCM PL011 UART"
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        self.map_pl011_uart();

        Ok(())
    }
}
