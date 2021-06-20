use crate::runtime::runtime_init;

global_asm!(include_str!("boot.S"));

#[no_mangle]
pub unsafe extern "C" fn __start_os() -> ! {
    runtime_init()
}
