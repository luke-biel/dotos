use core::{fmt, panic::PanicInfo};

use crate::{arch::arch_impl::cpu, common::statics::panic_console};

fn panic_print(args: fmt::Arguments) {
    use fmt::Write;
    unsafe {
        panic_console()
            .write_fmt(args)
            .expect("panic console write_fmt")
    };
}

#[macro_export]
macro_rules! panic_print {
    ($($arg:tt)*) => { $crate::panic::panic_print(format_args!($($arg)*)); };
}

#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
    if let Some(args) = info.message() {
        panic_print!("\nKernel panic: {}", args);
    } else {
        panic_print!("\nKernel panic");
    }

    cpu::park()
}
