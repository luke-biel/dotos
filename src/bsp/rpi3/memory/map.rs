pub const END: usize = 0xffff_ffff;
pub const GPIO_OFFSET: usize = 0x0020_0000;
pub const UART_OFFSET: usize = 0x0020_1000;

pub mod mmio {
    use crate::bsp::rpi3::memory::map::{GPIO_OFFSET, UART_OFFSET};

    pub const START: usize = 0x3f00_0000;
    pub const GPIO_START: usize = START + GPIO_OFFSET;
    pub const UART_START: usize = START + UART_OFFSET;
    pub const END: usize = 0x4000_ffff;
}
