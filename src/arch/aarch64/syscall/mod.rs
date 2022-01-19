use core::arch::global_asm;

global_asm!(include_str!("syscall.s"));

extern "Rust" {
    pub fn _exit(code: isize);
    pub fn _write(start: *const u8, len: usize);
}
