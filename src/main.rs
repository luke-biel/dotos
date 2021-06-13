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
unsafe extern "C" fn kernel_main() -> ! {
    clear_region(bss_section());
    UartConsole::init();
    println!("Hello, {}", 12);

    loop {}
}

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    abort()
}
