pub trait Driver {
    fn compat(&self) -> &'static str;
    unsafe fn init(&self) -> Result<(), &'static str> {
        Ok(())
    }
    unsafe fn late_init(&self) -> Result<(), &'static str> {
        Ok(())
    }
    fn register_irq_handler(&'static self) -> Result<(), &'static str> {
        Ok(())
    }
    fn virt_mmio_start_addr(&self) -> Option<usize> {
        None
    }
}

pub trait DriverManager {
    unsafe fn init_early_drivers(&self) -> Result<(), &'static str>;
    unsafe fn post_early_drivers(&self) -> Result<(), &'static str>;
    unsafe fn init_late_drivers(&self) -> Result<(), &'static str>;
    fn register_irq_handlers(&'static self) -> Result<(), &'static str>;
}
