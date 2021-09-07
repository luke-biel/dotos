use crate::common::memory::{Address, Physical};

pub const END: Address<Physical> = mmio::END;

pub mod mmio {
    use crate::common::memory::{Address, Physical};

    pub const PERIPHERAL_IC_START: Address<Physical> = Address::new(0x3F00_B200);
    pub const PERIPHERAL_IC_SIZE: usize = 0x24;

    pub const GPIO_START: Address<Physical> = Address::new(0x3F20_0000);
    pub const GPIO_SIZE: usize = 0xA0;

    pub const UART_START: Address<Physical> = Address::new(0x3F20_1000);
    pub const UART_SIZE: usize = 0x48;

    pub const LOCAL_IC_START: Address<Physical> = Address::new(0x4000_0000);
    pub const LOCAL_IC_SIZE: usize = 0x100;

    pub const END: Address<Physical> = Address::new(0x4001_0000);
}
