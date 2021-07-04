#![feature(global_asm)]
#![feature(asm)]
#![no_std]
#![no_main]

use crate::bsp::device_driver::PL011_UART;
use crate::bsp::raspberry_pi_3::driver::driver_manager;
use crate::common::driver::DriverManager;

mod arch;
mod bsp;
mod panic_handler;
mod common;

unsafe fn kernel_init() -> ! {
    let manager = driver_manager();
    for driver in manager.all().iter() {
        if let Err(err) = driver.init() {
            panic!("Error initializing driver {}: {}", driver.compat(), err);
        }
    }

    manager.post_device_driver_init();

    kernel_main()
}

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

    let uart = &PL011_UART;

    loop {
        let printme = uart.read_char_blocking();
        println!("Hello, {}", printme as char);
    }
}
