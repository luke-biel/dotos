#![allow(clippy::upper_case_acronyms)]
#![feature(core_intrinsics, global_asm, asm, stdsimd)]
#![no_std]
#![no_main]

use crate::bsp::driver::driver_manager;
use crate::common::driver::DriverManager;
use bsp::device_driver::bcm::bcm2xxx_pl011_uart::uart::UART;
use core::intrinsics::abort;
use core::panic::PanicInfo;

mod arch;
mod bsp;
mod common;
mod pointer_iter;
mod runtime;

#[allow(clippy::empty_loop)]
unsafe fn kernel_main() -> ! {
    println!(
        "> {} - v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    println!("> build time: {}", env!("BUILD_DATE"));
    println!("> git head: {}", env!("GIT_HASH"));

    println!("> drivers loaded:");
    for (i, driver) in driver_manager().all().iter().enumerate() {
        println!("> {}: {}", i, driver.compat())
    }

    let uart = UART::new(0x2020_1000usize);

    loop {
        let printme = uart.read_char_blocking();
        println!("Hello, {}", printme as char);
    }
}

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    // TODO: Reinit UART, print what happened
    abort()
}

pub(crate) unsafe fn kernel_init() -> ! {
    let manager = driver_manager();
    for driver in manager.all().iter() {
        if let Err(err) = driver.init() {
            panic!("Error initializing driver {}: {}", driver.compat(), err);
        }
    }

    manager.post_device_driver_init();

    kernel_main()
}
