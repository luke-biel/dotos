use crate::bsp::rpi::{GPIO, PL011_UART};
use crate::common::driver::{DeviceDriver, DriverManager};

static BSP_DRIVER_MANAGER: BspDriverManager = BspDriverManager {
    drivers: [&GPIO, &PL011_UART],
};

pub struct BspDriverManager {
    drivers: [&'static (dyn DeviceDriver + Sync); 2],
}

pub fn driver_manager() -> &'static impl DriverManager {
    &BSP_DRIVER_MANAGER
}

impl DriverManager for BspDriverManager {
    fn all(&self) -> &[&'static (dyn DeviceDriver + Sync)] {
        &self.drivers
    }

    fn post_device_driver_init(&self) {
        GPIO.map_pl011_uart()
    }
}
