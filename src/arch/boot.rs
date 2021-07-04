global_asm!(include_str!("boot.S"));

#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    crate::runtime::runtime_init()
}
