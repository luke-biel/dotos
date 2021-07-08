pub unsafe fn isb<T>(op: impl FnOnce() -> T) -> T {
    asm!("ISB", options(nostack));
    op()
}
