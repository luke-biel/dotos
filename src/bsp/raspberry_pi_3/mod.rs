pub mod driver;
pub mod io;
pub mod mem;

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;
