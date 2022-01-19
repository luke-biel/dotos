use core::arch::global_asm;

use crate::common::task::Task;

global_asm!(include_str!("task.s"));

#[derive(Default, Debug)]
#[repr(C)]
pub struct CpuContext {
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    pub fp: u64,
    pub sp: u64,
    pub pc: u64,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct PtRegs {
    pub registers: [u64; 31],
    pub sp: u64,
    pub pc: u64,
    pub pstate: u64,
}

extern "C" {
    pub fn cpu_switch_to(prev: *const Task, next: *const Task);
}
