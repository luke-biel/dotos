use core::fmt;
use crate::bsp::rpi::PL011_UART;

pub struct UartConsole;

impl fmt::Write for UartConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        PL011_UART.write_blocking(s);

        Ok(())
    }
}
