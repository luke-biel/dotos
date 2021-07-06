use crate::arch::cpu::spin_for_cycles;
use crate::bsp::device_driver::bcm::gpio::registers::{
    GPIORegisterBlock, GPFSEL1, GPPUDCLK0, GPPUP,
};
use crate::bsp::device_driver::WrappedPointer;
use tock_registers::interfaces::Writeable;

pub struct GpioInner {
    block: WrappedPointer<GPIORegisterBlock>,
}

impl GpioInner {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            block: WrappedPointer::new(mmio_start_addr),
        }
    }

    fn disable_pud(&self) {
        self.block.gppup.write(GPPUP::PUD::Off);
        spin_for_cycles(20_000);
        self.block
            .gppudclk0
            .write(GPPUDCLK0::PUDCLK14::AssertClock + GPPUDCLK0::PUDCLK15::AssertClock);
        spin_for_cycles(20_000);
        self.block.gppup.write(GPPUP::PUD::Off);
        self.block.gppudclk0.set(0);
    }

    pub fn map_pl011_uart(&self) {
        self.block
            .gpfsel1
            .write(GPFSEL1::FSEL15::AltFunc0 + GPFSEL1::FSEL14::AltFunc0);

        self.disable_pud();
    }
}
