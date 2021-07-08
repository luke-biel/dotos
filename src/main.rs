#![feature(global_asm)]
#![feature(asm)]
#![no_std]
#![no_main]

use crate::bsp::device_driver::PL011_UART;
use crate::bsp::raspberry_pi_3::driver::driver_manager;
use crate::common::driver::DriverManager;
use crate::arch::timer::Timer;

mod arch;
mod bsp;
mod common;
mod panic_handler;

unsafe fn kernel_init() -> ! {
    let manager = driver_manager();

    manager.init();
    manager.late_init();

    kernel_main()
}

unsafe fn kernel_main() -> ! {
    let timer = Timer;

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

    println!("time is {}s", timer.time_since_start().as_secs_f64());

    let uart = &PL011_UART;

    loop {
        let printme = uart.read_char_blocking();
        println!("Hello, {}", printme as char);
    }
}
