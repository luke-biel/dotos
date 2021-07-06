pub trait DeviceDriver {
    fn compat(&self) -> &'static str;

    unsafe fn init(&self) -> Result<(), &'static str> {
        Ok(())
    }

    unsafe fn late_init(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

pub trait DriverManager {
    fn all(&self) -> &[&'static (dyn DeviceDriver + Sync)];
    unsafe fn init(&self);
    unsafe fn late_init(&self);
}
