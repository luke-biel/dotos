#![feature(global_asm)]
#![feature(asm)]
#![feature(const_panic)]
#![no_std]
#![no_main]

use crate::arch::aarch64::exceptions::{current_privilege_level, print_state};
use crate::bsp::device_driver::PL011_UART;
use crate::bsp::raspberry_pi_3::driver::driver_manager;
use crate::common::driver::DriverManager;
use crate::arch::aarch64::memory::mmu::{MMU};
use crate::common::memory::interface::MMUInterface;

mod arch;
mod bsp;
mod common;
mod panic_handler;

unsafe fn kernel_init() -> ! {
    if let Err(e) = MMU.enable_mmu_and_caching() {
        panic!("MMU error: {:?}", e);
    }

    let manager = driver_manager();

    manager.init();
    manager.late_init();

    kernel_main()
}

unsafe fn kernel_main() -> ! {
    info!(
        "> {} - v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    info!("> build time: {}", env!("BUILD_DATE"));
    info!("> git head: {}", env!("GIT_HASH"));

    info!("> drivers loaded:");
    for (i, driver) in driver_manager().all().iter().enumerate() {
        info!("> {}: {}", i, driver.compat())
    }

    let (_, privilege_level) = current_privilege_level();
    info!("Current privilege level: {}", privilege_level);

    info!("Exception handling state:");
    print_state();

    let uart = &PL011_UART;
    let mut buf = [0u8; 512];
    let mut idx = 0;

    loop {
        let c = uart.read_char_blocking();
        if c == b'\n' {
            info!("\n{}", core::str::from_utf8_unchecked(&buf[0..=idx]));
        } else {
            buf[idx] = c;
            idx += 1;
            print!("{}", c as char);
        }
    }
}
