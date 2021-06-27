pub trait DeviceDriver {
    fn compat(&self) -> &'static str;
    unsafe fn init(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

pub trait DriverManager {
    fn all(&self) -> &[&'static (dyn DeviceDriver + Sync)];

    fn post_device_driver_init(&self);
}
