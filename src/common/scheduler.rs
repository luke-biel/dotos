use core::mem::{forget, transmute, MaybeUninit};

use crate::{
    arch::arch_impl::task::CpuContext,
    common::{
        sync::IRQSafeNullLock,
        task::{Task, TaskState},
    },
};

pub const INIT_TASK: Task = Task {
    context: CpuContext::zero(),
    state: TaskState::Running,
    counter: 0,
    priority: 1,
    preempt_count: 0,
};

pub static SCHEDULER: IRQSafeNullLock<Scheduler<64>> =
    IRQSafeNullLock::new(Scheduler::new().with(INIT_TASK));

pub struct Scheduler<const C: usize> {
    tasks: [Option<Task>; C],
    count: usize,
    current: Option<usize>,
}

impl<const C: usize> Scheduler<C> {
    const fn new() -> Self {
        const INIT_VAL: Option<Task> = None;
        let tasks = [INIT_VAL; C];

        Self {
            tasks,
            count: 0,
            current: None,
        }
    }

    const fn with(mut self, task: Task) -> Self {
        if self.count + 1 >= C {
            panic!("task limit reached, cannot instantiate new task");
        }
        self.tasks[self.count] = Some(task);
        self.count += 1;
        self
    }
}
