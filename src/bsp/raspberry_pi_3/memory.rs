use core::cell::UnsafeCell;

extern "Rust" {
    static __bss_start: UnsafeCell<i64>;
    static __bss_end_exclusive: UnsafeCell<i64>;
}

pub const GPIO_OFFSET: usize = 0x20_0000;
pub const UART_OFFSET: usize = 0x20_1000;

const MMIO_BASE: usize = 0x3F00_0000;

pub const GPIO_START: usize = MMIO_BASE + GPIO_OFFSET;
pub const UART_START: usize = MMIO_BASE + UART_OFFSET;
