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

    pub const TIMER_START: Address<Physical> = Address::new(0x3F00_3000);
    pub const TIMER_SIZE: usize = 0x1c;

    pub const END: Address<Physical> = Address::new(0x4001_0000);
}

pub mod user {
    use crate::{
        arch::arch_impl::memory::mmu::Granule64KB,
        bsp::device::memory::map::mmio::PERIPHERAL_IC_START,
        common::memory::{Address, Physical},
    };

    pub const LOW_MEMORY: Address<Physical> = Address::new(0x1020_0000);
    pub const HIGH_MEMORY: Address<Physical> = PERIPHERAL_IC_START;

    pub const PAGING_MEMORY_SIZE: usize = HIGH_MEMORY.addr() - LOW_MEMORY.addr();
    pub const PAGE_COUNT: usize = PAGING_MEMORY_SIZE / Granule64KB::SIZE;
}
