use core::fmt;

use crate::bsp::{
    device_driver::bcm::{bcm2xxx_gpio::Gpio, bcm2xxx_pl011_uart::PL011Uart},
    rpi3::{driver::BSPDriverManager, memory::map::mmio},
};

pub static GPIO_DRIVER: Gpio = unsafe { Gpio::new(mmio::GPIO_START.addr()) };
pub static UART_DRIVER: PL011Uart = unsafe { PL011Uart::new(mmio::UART_START.addr()) };
pub static INTERRUPT_CONTROLLER: InterruptController = unsafe {
    InterruptController::new(
        mmio::LOCAL_IC_START.addr(),
        mmio::PERIPHERAL_IC_START.addr(),
    )
};

pub static BSP_DRIVER_MANAGER: BSPDriverManager<3> = BSPDriverManager {
    drivers: [&GPIO_DRIVER, &UART_DRIVER, &INTERRUPT_CONTROLLER],
};

pub use self::UART_DRIVER as CONSOLE;
use crate::bsp::device_driver::bcm::{
    bcm2xxx_gpio::GpioInner,
    bcm2xxx_interrupt_controller::InterruptController,
    bcm2xxx_pl011_uart::PL011UartInner,
};

pub const LOG_LEVEL: usize = 4;

pub unsafe fn panic_console() -> impl fmt::Write {
    let mut gpio = GpioInner::new(mmio::GPIO_START.addr());
    let mut uart = PL011UartInner::new(mmio::UART_START.addr());

    gpio.map_pl011_uart();
    uart.init();
    uart
}
