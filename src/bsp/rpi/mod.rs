pub mod driver;
pub mod io;
pub mod mem;

use super::device_driver;
use crate::bsp::rpi::mem::{GPIO_START, UART_START};

pub static GPIO: device_driver::GPIO = unsafe { device_driver::GPIO::new(GPIO_START) };
pub static PL011_UART: device_driver::UART = unsafe { device_driver::UART::new(UART_START) };
