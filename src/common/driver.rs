pub trait Driver {
    fn compat(&self) -> &'static str;
    unsafe fn init(&self) -> Result<(), &'static str> {
        Ok(())
    }
    unsafe fn late_init(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

pub trait DriverManager {
    unsafe fn init(&self) -> Result<(), &'static str>;
    unsafe fn late_init(&self) -> Result<(), &'static str>;
}
