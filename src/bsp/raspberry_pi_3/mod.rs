pub mod io;
pub mod mem;
pub mod driver;

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;
