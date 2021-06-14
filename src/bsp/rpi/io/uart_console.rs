use core::fmt;

use crate::bsp::device_driver::bcm::bcm2xxx_gpio::GPIO;
use crate::bsp::device_driver::bcm::bcm2xxx_pl011_uart::uart::UART;

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

        gpio.map_pl011_uart();

        uart.map_pl011_uart()
    }
}
