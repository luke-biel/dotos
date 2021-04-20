use core::fmt;

use crate::arch::cpu;
use crate::bsp::device_driver::bcm::bcm2xxx_gpio::{GPIORegisterBlock, GPPUP};
use crate::bsp::device_driver::bcm::bcm2xxx_pl011_uart::UARTRegisterBlock;
use crate::bsp::rpi::io::{mmio_read, mmio_write};
use crate::bsp::rpi::mem::{
    GPPUDCLK0, UART0_CR, UART0_DR, UART0_FBRD, UART0_FR, UART0_IBRD, UART0_ICR, UART0_IMSC,
    UART0_LCRH,
};

pub struct UartConsole;

impl fmt::Write for UartConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            while mmio_read(UART0_FR) & (1 << 5) > 0 {}
            mmio_write(UART0_DR, c as u32)
        }

        Ok(())
    }
}

impl UartConsole {
    pub fn init() {
        let gpio_rb = 0x2020_0000usize as *const GPIORegisterBlock;
        let uart_rb = 0x2020_1000usize as *const UARTRegisterBlock;

        unsafe { (*uart_rb).cr.set(0) };
        // mmio_write(UART0_CR, 0x00000000);

        unsafe { (*gpio_rb).gppup.write(GPPUP::PUD::Off) };
        // mmio_write(GPPUD, 0x00000000);
        cpu::sleep(150);

        mmio_write(GPPUDCLK0, (1 << 14) | (1 << 15));
        cpu::sleep(150);

        mmio_write(GPPUDCLK0, 0x00000000);

        mmio_write(UART0_ICR, 0x7FF);

        mmio_write(UART0_IBRD, 1);
        mmio_write(UART0_FBRD, 40);

        mmio_write(UART0_LCRH, (1 << 4) | (1 << 5) | (1 << 6));

        mmio_write(
            UART0_IMSC,
            (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10),
        );

        mmio_write(UART0_CR, (1 << 0) | (1 << 8) | (1 << 9));
    }
}
