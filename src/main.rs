#![allow(clippy::upper_case_acronyms)]
#![feature(core_intrinsics, global_asm, asm)]
#![no_std]
#![no_main]

use core::cmp::Ordering;
use core::intrinsics::abort;
use core::panic::PanicInfo;

use bsp::device_driver::bcm::bcm2xxx_pl011_uart::uart::UART;

use crate::arch::{Int, IntPtr, UInt};
use crate::bsp::io::uart_console::UartConsole;
use crate::bsp::mem::bss_section;
use crate::common::mem::clear_region;

mod arch;
mod bsp;
mod c;
mod common;
mod pointer_iter;

global_asm!(
    r#"
.section ".text._start"

.global _start

_start:
    ldr     r1, =_start
    mov     sp, r1
    bl      kernel_main
"#
);

#[no_mangle]
#[allow(clippy::empty_loop)]
unsafe extern "C" fn kernel_main() -> ! {
    clear_region(bss_section());
    UartConsole::init();

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

