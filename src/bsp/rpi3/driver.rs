use crate::{
    common::driver::{Driver, DriverManager},
    info,
};

pub struct BSPDriverManager<const E: usize, const L: usize> {
    pub early_drivers: [&'static (dyn Driver + Sync); E],
    pub late_drivers: [&'static (dyn Driver + Sync); L],
}

impl<const T: usize, const L: usize> DriverManager for BSPDriverManager<T, L> {
    unsafe fn init_early_drivers(&self) -> Result<(), &'static str> {
        for driver in self.early_drivers {
            driver.init()?;
        }

        Ok(())
    }

    unsafe fn post_early_drivers(&self) -> Result<(), &'static str> {
        for driver in self.early_drivers {
            driver.late_init()?;
        }

        Ok(())
    }

    unsafe fn init_late_drivers(&self) -> Result<(), &'static str> {
        for driver in self.late_drivers {
            driver.init()?;
        }

        Ok(())
    }

    fn register_irq_handlers(&'static self) -> Result<(), &'static str> {
        for driver in self.early_drivers {
            driver.register_irq_handler()?;
        }
        for driver in self.late_drivers {
            driver.register_irq_handler()?;
        }

        Ok(())
    }
}

impl<const T: usize, const L: usize> BSPDriverManager<T, L> {
    pub fn print_status(&self) {
        info!("drivers loaded:");
        for (i, driver) in self.early_drivers.iter().enumerate() {
            info!("  {}): {}", i, driver.compat());
        }
        for (i, driver) in self.late_drivers.iter().enumerate() {
            info!("  {}): {}", i, driver.compat());
        }
    }
}
