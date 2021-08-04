use core::fmt;

use crate::bsp::raspberry_pi_3::io::uart_console::UartConsole as Console;

#[macro_export]
macro_rules! println {
    () => (crate::print!("\n"));
    ($($arg:tt)*) => (crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! info {
    () => (crate::print!("[I {}s]: null\n", arch::aarch64::timer::Timer.time_since_start().as_secs_f64()));
    ($($arg:tt)*) => (crate::print!("[I {}s]: {}\n", arch::aarch64::timer::Timer.time_since_start().as_secs_f64(), format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::common::io::_print(format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    Console.write_fmt(args).unwrap();
}
