pub trait DeviceDriver {
    fn compat() -> &'static str;
    unsafe fn init(&self) -> Result<(), &'static str>;
}
