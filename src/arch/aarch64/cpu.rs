use cortex_a::asm::{nop, wfe};

pub fn spin_for_cycles(count: u32) {
    for _ in 0..count {
        nop()
    }
}

pub fn wait_forever() -> ! {
    loop {
        wfe()
    }
}
