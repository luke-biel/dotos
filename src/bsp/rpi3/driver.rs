use crate::common::driver::{Driver, DriverManager};

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
}
