use crate::bsp::device_driver::{GPIO, PL011_UART};
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

    unsafe fn init(&self) {
        for driver in &self.drivers {
            if let Err(error) = driver.init() {
                panic!("Error initializing driver {}: {}", driver.compat(), error);
            }
        }
    }

    unsafe fn late_init(&self) {
        for driver in &self.drivers {
            if let Err(error) = driver.late_init() {
                panic!(
                    "Error running late_init for driver {}: {}",
                    driver.compat(),
                    error
                );
            }
        }
    }
}
