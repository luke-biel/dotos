#![allow(clippy::upper_case_acronyms)]
#![feature(core_intrinsics, global_asm, asm)]
#![no_std]
#![no_main]

use core::intrinsics::abort;
use core::panic::PanicInfo;
use bsp::device_driver::bcm::bcm2xxx_pl011_uart::uart::UART;
use crate::bsp::io::uart_console::UartConsole;
use crate::bsp::mem::bss_section;
use crate::common::mem::zero_region_volatile;

mod arch;
mod bsp;
mod common;
mod pointer_iter;
mod runtime;

#[allow(clippy::empty_loop)]
unsafe fn kernel_main() -> ! {
    println!("> {} - v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("> build time: {}", env!("BUILD_DATE"));
    println!("> git head: {}", env!("GIT_HASH"));

    let uart = UART::new(0x2020_1000usize);

    loop {
        if let Some(printme) = uart.read_char() {
            println!("Hello, {}", printme as char);
        }
    }
}

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    abort()
}

pub(crate) unsafe fn kernel_init() -> ! {
    UartConsole::init();

    kernel_main()
}
