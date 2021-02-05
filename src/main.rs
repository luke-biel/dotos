#![feature(core_intrinsics, global_asm, asm)]
#![no_std]
#![no_main]

use core::intrinsics::abort;
use core::panic::PanicInfo;

use crate::board_support::io::uart_console::UartConsole;
use crate::board_support::mem::bss_section;
use crate::common::mem::clear_region;

mod arch;
mod board_support;
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
pub unsafe extern "C" fn kernel_main() -> ! {
    clear_region(bss_section());
    UartConsole::init();
    println!("Hello, baby");

    loop {}
}

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    abort()
}
