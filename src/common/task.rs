use num_derive::{FromPrimitive, ToPrimitive};

use crate::arch::arch_impl::task::CpuContext;

pub struct Task {
    pub context: CpuContext,
    pub state: TaskState,
    /// Task time on CPU countdown, each timer tick will lower it down
    pub counter: u64,
    pub priority: u64,
    /// Task is performing critical work and cannot be dispossessed
    pub preempt_count: u64,
}

#[repr(C)]
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, PartialEq)]
pub enum TaskState {
    Running = 0,
}

impl Task {
    pub fn store(&self) {
        unsafe { self.context.store() }
    }

    pub fn restore(&self) {
        unsafe { self.context.restore() }
    }
}
