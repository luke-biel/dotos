#![allow(dead_code)]

use core::intrinsics::{volatile_load, volatile_store};

pub mod uart_console;

const MMIO_BASE: u32 = 0x20000000;
const GPIO_BASE: u32 = MMIO_BASE + 0x200000;
const GPPUD: u32 = GPIO_BASE + 0x94;
const GPPUDCLK0: u32 = GPIO_BASE + 0x98;
const UART0_BASE: u32 = GPIO_BASE + 0x1000;
const UART0_DR: u32 = UART0_BASE + 0x00;
const UART0_RSRECR: u32 = UART0_BASE + 0x04;
const UART0_FR: u32 = UART0_BASE + 0x18;
const UART0_ILPR: u32 = UART0_BASE + 0x20;
const UART0_IBRD: u32 = UART0_BASE + 0x24;
const UART0_FBRD: u32 = UART0_BASE + 0x28;
const UART0_LCRH: u32 = UART0_BASE + 0x2C;
const UART0_CR: u32 = UART0_BASE + 0x30;
const UART0_IFLS: u32 = UART0_BASE + 0x34;
const UART0_IMSC: u32 = UART0_BASE + 0x38;
const UART0_RIS: u32 = UART0_BASE + 0x3C;
const UART0_MIS: u32 = UART0_BASE + 0x40;
const UART0_ICR: u32 = UART0_BASE + 0x44;
const UART0_DMACR: u32 = UART0_BASE + 0x48;
const UART0_ITCR: u32 = UART0_BASE + 0x80;
const UART0_ITIP: u32 = UART0_BASE + 0x84;
const UART0_ITOP: u32 = UART0_BASE + 0x88;
const UART0_TDR: u32 = UART0_BASE + 0x8C;
const MBOX_BASE: u32 = MMIO_BASE + 0xB880;
const MBOX_READ: u32 = MBOX_BASE + 0x00;
const MBOX_STATUS: u32 = MBOX_BASE + 0x18;
const MBOX_WRITE: u32 = MBOX_BASE + 0x20;

fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *mut u32) }
}
