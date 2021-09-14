#[repr(C)]
pub struct CpuContext {
    // x19 - x28, task cannot save state of registers < x19 (they can change between jumps, per ARM spec)
    registers: [u64; 10],
    fp: u64,
    sp: u64,
    pc: u64,
}

impl CpuContext {
    pub const fn zero() -> Self {
        Self {
            registers: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            fp: 0,
            sp: 0,
            pc: 0
        }
    }
}
