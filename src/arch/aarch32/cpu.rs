use core::arch::arm::__nop;

pub fn sleep(count: u32) {
    unsafe {
        asm!(r#"
        1:
            subs {0}, {0}, #1;
            bne 1b
        "#, in(reg) count);
    }
}

pub fn nop() {
    unsafe {
        __nop()
    }
}
