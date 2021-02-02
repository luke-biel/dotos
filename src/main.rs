#![feature(core_intrinsics, global_asm, asm)]
#![no_std]
#![no_main]

use crate::board_support::io::uart_console::UartConsole;
use core::intrinsics::{abort, volatile_load, volatile_store};
use core::panic::PanicInfo;

mod arch;
mod board_support;
mod common;

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
    UartConsole::init();
    println!("Hello, baby");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { abort() }
}
