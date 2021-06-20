use core::fmt;

use crate::bsp::device_driver::bcm::bcm2xxx_gpio::GPIO;
use crate::bsp::device_driver::bcm::bcm2xxx_pl011_uart::uart::UART;
use crate::bsp::rpi::mem::{GPIO_START, UART_START};

pub struct UartConsole;

impl fmt::Write for UartConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let uart = unsafe { UART::new(UART_START) };

        uart.write_blocking(s);

        Ok(())
    }
}

impl UartConsole {
    pub fn init() {
        let gpio = unsafe { GPIO::new(GPIO_START) };
        let uart = unsafe { UART::new(UART_START) };

        gpio.map_pl011_uart();

        uart.map_pl011_uart()
    }
}
