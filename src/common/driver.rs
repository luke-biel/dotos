pub trait DeviceDriver {
    unsafe fn init(&self) -> Result<(), &'static str>;
}
