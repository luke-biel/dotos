use core::fmt;

use crate::bsp::device_driver::bcm::bcm2xxx_gpio::{GPIO};
use crate::bsp::device_driver::bcm::bcm2xxx_pl011_uart::{UART};
use crate::bsp::rpi::io::{mmio_read, mmio_write};
use crate::bsp::rpi::mem::{UART0_FR, UART0_DR};

pub struct UartConsole;

impl fmt::Write for UartConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let uart = unsafe { UART::new(0x2020_1000usize) };

        uart.write_blocking(s);

        Ok(())
    }
}

impl UartConsole {
    pub fn init() {
        let gpio = unsafe { GPIO::new(0x2020_0000usize) };
        let uart = unsafe { UART::new(0x2020_1000usize) };

        uart.clear_cr();

        gpio.init_pl011_uart();

        uart.init_pl011_uart()
    }
}
