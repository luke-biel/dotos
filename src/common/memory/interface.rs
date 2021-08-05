pub trait MMUInterface {
    unsafe fn enable_mmu_and_caching(&self) -> Result<(), &'static str>;
    fn is_enabled(&self) -> bool;
}
