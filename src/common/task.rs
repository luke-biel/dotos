use num_derive::{FromPrimitive, ToPrimitive};

use crate::arch::arch_impl::task::{cpu_switch_to, CpuContext};

#[derive(Default, Debug)]
#[repr(C)]
pub struct Task {
    pub context: CpuContext,
    pub state: TaskState,
    /// Task time on CPU countdown, each timer tick will lower it down
    pub counter: u64,
    pub priority: u64,
    /// Task is performing critical work and cannot be dispossessed
    pub preempt_count: u64,
}

#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub enum TaskState {
    Running = 0,
}

impl Default for TaskState {
    fn default() -> Self {
        Self::Running
    }
}

impl Task {
    pub unsafe fn cpu_switch_to(prev: &Task, next: &Task) {
        cpu_switch_to(prev as *const _, next as *const _)
    }
}
