use core::fmt;

use crate::bsp::raspberry_pi_3::io::uart_console::UartConsole as Console;

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::common::io::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    Console.write_fmt(args).unwrap();
}
