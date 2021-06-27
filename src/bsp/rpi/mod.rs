pub mod io;
pub mod mem;
pub mod driver;

use super::device_driver;
use crate::bsp::rpi::mem::{GPIO_START, UART_START};

static GPIO: device_driver::GPIO = unsafe { device_driver::GPIO::new(GPIO_START) };
static PL011_UART: device_driver::UART = unsafe { device_driver::UART::new(UART_START) };
