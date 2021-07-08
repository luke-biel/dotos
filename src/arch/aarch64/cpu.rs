pub fn spin_for_cycles(count: u32) {
    for _ in 0..count {
        unsafe { asm!("nop") }
    }
}

pub fn wait_forever() -> ! {
    loop {
        unsafe { asm!("wfe") }
    }
}
