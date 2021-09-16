use core::fmt;

use crate::common::{serial_console::Write, statics::CONSOLE};

pub fn _print(args: fmt::Arguments) {
    CONSOLE.write_fmt(args).expect("default console write_fmt")
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => { $crate::log::_print(format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n"); };
    ($($arg:tt)*) => {
        $crate::log::_print(format_args_nl!($($arg)*));
    };
}

#[macro_export]
macro_rules! log {
    ($log_kw:expr, $log_lv:expr, $s:expr) => {{
        use crate::common::time::clock::ClockManager as _;

        let ts = $crate::common::statics::CLOCK_TIMER.uptime();
        let sts = ts.subsec_micros();

        if $crate::common::statics::LOG_LEVEL >= $log_lv {
            $crate::log::_print(
                format_args_nl!(
                    concat!("(", $log_kw, ")", "[{:>3}.{:03}{:03}] ", $s),
                    ts.as_secs(),
                    sts / 1000,
                    sts % 1000
                )
            );
        };
    }};
    ($log_kw:expr, $log_lv:expr, $fs:expr, $($arg:tt)*) => {{
        use crate::common::time::clock::ClockManager as _;

        let ts = $crate::common::statics::CLOCK_TIMER.uptime();
        let sts = ts.subsec_micros();

        if $crate::common::statics::LOG_LEVEL >= $log_lv {
            $crate::log::_print(
                    format_args_nl!(
                    concat!("(", $log_kw, ")", "[{:>3}.{:03}{:03}] ", $fs),
                    ts.as_secs(),
                    sts / 1000,
                    sts % 1000,
                    $($arg)*
                )
            );
        };
    }};
}

#[macro_export]
macro_rules! trace {
    ($s:expr) => { $crate::log!("T", 4, $s); };
    ($fs:expr, $($arg:tt)*) => {
        $crate::log!("T", 4, $fs, $($arg)*)
    };
}

#[macro_export]
macro_rules! debug {
    ($s:expr) => { $crate::log!("D", 3, $s) };
    ($fs:expr, $($arg:tt)*) => {
        $crate::log!("D", 3, $fs, $($arg)*)
    };
}

#[macro_export]
macro_rules! info {
    ($s:expr) => { $crate::log!("I", 2, $s) };
    ($fs:expr, $($arg:tt)*) => {
        $crate::log!("I", 2, $fs, $($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($s:expr) => { $crate::log!("W", 1, $s) };
    ($fs:expr, $($arg:tt)*) => {
        $crate::log!("W", 1, $fs, $($arg)*)
    };
}

#[macro_export]
macro_rules! error {
    ($s:expr) => { $crate::log!("E", 0, $s) };
    ($fs:expr, $($arg:tt)*) => {
        $crate::log!("E", 0, $fs, $($arg)*)
    };
}
