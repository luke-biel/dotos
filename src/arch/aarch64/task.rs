#[derive(Default)]
#[repr(C)]
pub struct CpuContext {
    // x19 - x28, task cannot save state of registers < x19 (they can change between jumps, per ARM spec)
    pub registers: [u64; 10],
    pub fp: u64,
    pub sp: u64,
    pub pc: u64,
}

impl CpuContext {
    pub const fn zero() -> Self {
        Self {
            registers: [0; 10],
            fp: 0,
            sp: 0,
            pc: 0,
        }
    }

    pub unsafe fn store(&self) {
        asm!(
            "mov x9, sp",
            "stp x19, x20, [x0], #16",
            "stp x21, x22, [x0], #16",
            "stp x23, x24, [x0], #16",
            "stp x25, x26, [x0], #16",
            "stp x27, x28, [x0], #16",
            "stp x29, x9, [x0], #16",
            "str x30, [x0]"
        )
    }

    pub unsafe fn restore(&self) {
        asm!(
            "ldp x19, x20, [x0], #16",
            "ldp x21, x22, [x0], #16",
            "ldp x23, x24, [x0], #16",
            "ldp x25, x26, [x0], #16",
            "ldp x27, x28, [x0], #16",
            "ldp x29, x9, [x0], #16",
            "ldr x30, [x0]",
            "mov sp, x9"
        )
    }
}
