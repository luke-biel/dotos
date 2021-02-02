pub fn sleep(mut count: u32) {
    unsafe {
        asm!(r#"
        1:
            subs {0}, {0}, #1;
            bne 1b
        "#, inout(reg) count);
    }
}
