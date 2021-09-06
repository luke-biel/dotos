use crate::{
    common::driver::{Driver, DriverManager},
    info,
};

pub struct BSPDriverManager<const T: usize> {
    pub drivers: [&'static (dyn Driver + Sync); T],
}

impl<const T: usize> DriverManager for BSPDriverManager<T> {
    unsafe fn init(&self) -> Result<(), &'static str> {
        for driver in self.drivers {
            driver.init()?;
        }

        Ok(())
    }

    unsafe fn late_init(&self) -> Result<(), &'static str> {
        for driver in self.drivers {
            driver.late_init()?;
        }

        Ok(())
    }

    fn register_irq_handlers(&'static self) -> Result<(), &'static str> {
        for driver in self.drivers {
            driver.register_irq_handler()?;
        }

        Ok(())
    }
}

impl<const T: usize> BSPDriverManager<T> {
    pub fn print_status(&self) {
        info!("drivers loaded:");
        for (i, driver) in self.drivers.iter().enumerate() {
            info!("  {}): {}", i, driver.compat());
        }
    }
}
