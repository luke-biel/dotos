use tock_registers::registers::{Readable, Writeable};

use crate::bsp::device_driver::bcm::bcm2xxx_pl011_uart::{CR, FR, LCRH, UARTRegisterBlock};
use crate::bsp::device_driver::WrappedPointer;

struct UARTInner {
    block: WrappedPointer<UARTRegisterBlock>,
}

pub struct UART {
    inner: UARTInner,
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
    }

    fn flush(&self) {
        while self.block.fr.matches_all(FR::BUSY::SET) {
            unsafe { asm!("nop") }; // FIXME: this is cpu arch specific, move it
        }
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

    fn read_char(&self) -> Option<u8> {
        if self.block.fr.matches_all(FR::RXFE::Empty) {
            return None
        }

        Some(self.block.dr.get() as u8)
    }
}

impl UART {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: UARTInner::new(mmio_start_addr),
        }
    }

    pub fn map_pl011_uart(&self) {
        self.inner.map_pl011_uart()
    }

    pub fn write_blocking(&self, s: &str) {
        self.inner.write_blocking(s)
    }

    pub fn read_char(&self) -> Option<u8> {
        self.inner.read_char()
    }
}
