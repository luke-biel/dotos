use crate::common::task::Task;

#[derive(Default, Debug)]
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
}

global_asm!(include_str!("task.s"));

extern "C" {
    pub fn cpu_switch_to(prev: *const Task, next: *const Task);
}
