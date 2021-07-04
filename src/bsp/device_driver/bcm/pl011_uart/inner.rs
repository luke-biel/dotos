use tock_registers::interfaces::{Readable, Writeable};
use crate::bsp::device_driver::WrappedPointer;
use super::registers::{FR,ICR, IBRD, FBRD, LCRH, CR, UARTRegisterBlock};

pub struct UartInner {
    block: WrappedPointer<UARTRegisterBlock>,
}

impl UartInner {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            block: WrappedPointer::new(mmio_start_addr),
        }
    }

    pub fn write_blocking(&self, s: &str) {
        for c in s.chars() {
            while self.block.fr.matches_all(FR::TXFF::Full) {}
            self.block.dr.set(c as u32);
        }
    }

    pub  fn read_char_blocking(&self) -> u8 {
        while self.block.fr.matches_all(FR::RXFE::Empty) {
            unsafe { asm!("nop") }
        }

        let c = self.block.dr.get() as u8;

        if c == b'\r' {
            b'\n'
        } else {
            c
        }
    }

    pub  fn flush(&self) {
        while self.block.fr.matches_all(FR::BUSY::SET) {
            unsafe { asm!("nop") }
        }
    }

    pub fn map_pl011_uart(&self) {
        self.flush();

        self.block.cr.set(0);

        self.block.icr.write(ICR::ALL::CLEAR);

        self.block.ibrd.write(IBRD::BAUD_DIVINT.val(26));
        self.block.fbrd.write(FBRD::BAUD_DIVFRAC.val(3));

        self.block.lcrh.write(LCRH::FEN::Enabled + LCRH::WLEN::Len8);

        self.block
            .cr
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);
    }
}
