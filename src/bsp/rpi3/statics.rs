use core::fmt;

use crate::bsp::{
    device_driver::bcm::{bcm2xxx_gpio::Gpio, bcm2xxx_pl011_uart::PL011Uart},
    rpi3::{driver::BSPDriverManager, memory::map::mmio},
};

pub static GPIO_DRIVER: Gpio =
    unsafe { Gpio::new(MMIODescriptor::new(mmio::GPIO_START, mmio::GPIO_SIZE)) };
pub static UART_DRIVER: PL011Uart =
    unsafe { PL011Uart::new(MMIODescriptor::new(mmio::UART_START, mmio::UART_SIZE)) };
pub static INTERRUPT_CONTROLLER: InterruptController = unsafe {
    InterruptController::new(
        MMIODescriptor::new(mmio::LOCAL_IC_START, mmio::LOCAL_IC_SIZE),
        MMIODescriptor::new(mmio::PERIPHERAL_IC_START, mmio::PERIPHERAL_IC_SIZE),
    )
};

pub static BSP_DRIVER_MANAGER: BSPDriverManager<2, 1> = BSPDriverManager {
    early_drivers: [&GPIO_DRIVER, &UART_DRIVER],
    late_drivers: [&INTERRUPT_CONTROLLER],
};

pub use self::UART_DRIVER as CONSOLE;
use crate::{
    arch::arch_impl::cpu::park,
    bsp::device_driver::bcm::{
        bcm2xxx_gpio::GpioInner,
        bcm2xxx_interrupt_controller::InterruptController,
        bcm2xxx_pl011_uart::PL011UartInner,
    },
    common::{driver::Driver, memory::mmu::descriptors::MMIODescriptor},
};

pub const LOG_LEVEL: usize = 4;

pub unsafe fn panic_console() -> impl fmt::Write {
    let mut gpio = GpioInner::new(mmio::GPIO_START.addr());
    let mut uart = PL011UartInner::new(mmio::UART_START.addr());

    gpio.init(None).unwrap_or_else(|_| park());
    uart.init(None);
    gpio.map_pl011_uart();

    uart
}
