pub fn sleep_for_cycles(count: u32) {
    unsafe {
        asm!(r#"
        1:
            subs {0:w}, {0:w}, #1;
            bne 1b
        "#, in(reg) count);
    }
}

pub fn wait_forever() -> ! {
    loop {
        unsafe { asm!("wfe") }
    }
}
