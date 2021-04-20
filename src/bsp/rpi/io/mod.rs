use core::ptr::{read_volatile, write_volatile};

pub mod uart_console;

fn mmio_write(reg: u32, val: u32) {
    unsafe { write_volatile(reg as *mut u32, val) }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { read_volatile(reg as *mut u32) }
}
