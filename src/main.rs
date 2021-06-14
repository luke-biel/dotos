#![allow(clippy::upper_case_acronyms)]
#![feature(core_intrinsics, global_asm, asm)]
#![no_std]
#![no_main]

use core::intrinsics::abort;
use core::panic::PanicInfo;

use crate::bsp::io::uart_console::UartConsole;
use crate::bsp::mem::bss_section;
use crate::common::mem::clear_region;

mod arch;
mod bsp;
mod common;
mod pointer_iter;

use bsp::device_driver::bcm::bcm2xxx_pl011_uart::uart::UART;
use crate::arch::{UInt, IntPtr, Int};
use core::cmp::Ordering;

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

#[no_mangle]
unsafe fn memcmp(ptr1: IntPtr, ptr2: IntPtr, num: UInt) -> Int {
    for i in 0..num {
        let left = ptr1 as *const u8;
        let right = ptr2 as *const u8;

        match (*left.offset(i as isize)).cmp(&*right.offset(i as isize)) {
            Ordering::Less => return -1,
            Ordering::Equal => (),
            Ordering::Greater => return 1,
        }
    }

    0
}
