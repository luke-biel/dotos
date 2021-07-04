pub mod boot;
pub mod cpu;

pub fn wait_forever() -> ! {
    loop {
        unsafe { asm!("wfe") }
    }
}
