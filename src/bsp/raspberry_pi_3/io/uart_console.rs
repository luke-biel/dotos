use crate::bsp::device_driver::PL011_UART;
use core::fmt;

pub struct UartConsole;

impl fmt::Write for UartConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        PL011_UART.write_blocking(s);

        Ok(())
    }
}
