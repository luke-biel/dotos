use core::hint::unreachable_unchecked;

pub unsafe fn wfe() {
    asm!("wfe")
}

pub unsafe fn nop() {
    asm!("nop")
}

pub unsafe fn eret() -> ! {
    asm!("eret");
    unreachable_unchecked()
}
