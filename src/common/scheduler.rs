use core::mem::size_of;

use crate::{
    arch::arch_impl::{
        cpu::exception::{
            asynchronous::{mask_irq, unmask_irq},
            return_from_fork,
        },
        task::{CpuContext, PtRegs},
    },
    bsp::device_driver::WrappedPointer,
    common::{
        memory::mmu::next_free_page,
        sync::{IRQSafeNullLock, Mutex},
        task::{Task, TaskState},
        time::scheduling::TickCallbackHandler,
    },
};

pub static SCHEDULER: Scheduler<64> = Scheduler::new();
pub static mut INIT_TASK: Task = Task {
    context: CpuContext {
        x19: 0,
        x20: 0,
        x21: 0,
        x22: 0,
        x23: 0,
        x24: 0,
        x25: 0,
        x26: 0,
        x27: 0,
        x28: 0,
        fp: 0,
        sp: 0,
        pc: 0,
    },
    state: TaskState::Running,
    counter: 0,
    priority: 0,
    preempt_count: 0,
    stack: 0,
};

struct SchedulerInner<const C: usize> {
    tasks: heapless::Vec<WrappedPointer<Task>, C>,
    current: usize,
}

pub struct Scheduler<const C: usize> {
    inner: IRQSafeNullLock<SchedulerInner<C>>,
}

impl<const C: usize> SchedulerInner<C> {
    const fn new() -> Self {
        Self {
            tasks: heapless::Vec::new(),
            current: 0,
        }
    }

    pub fn init(&mut self) {
        unsafe {
            self.tasks
                .push(WrappedPointer::new(&mut INIT_TASK as *mut Task as usize))
                .expect("push init task")
        };
    }

    fn push_task(&mut self, task: WrappedPointer<Task>) {
        if self.tasks.push(task).is_err() {
            panic!("task cache is full")
        }
    }

    fn current(&mut self) -> Option<&mut WrappedPointer<Task>> {
        self.tasks.get_mut(self.current)
    }

    fn schedule(&mut self) {
        self.preempt_disable();

        let next = loop {
            let max = self
                .tasks
                .iter()
                .enumerate()
                .filter(|(_, def)| def.state == TaskState::Running)
                .max_by(|(_, t1), (_, t2)| t1.counter.cmp(&t2.counter));

            match max {
                Some((idx, ptr)) if ptr.counter > 0 => break idx,
                _ => {
                    for task in self.tasks.iter_mut() {
                        task.counter = task.priority
                    }
                }
            }
        };

        self.switch_to(next);

        self.preempt_enable();
    }

    fn preempt_disable(&mut self) {
        if let Some(current) = self.current() {
            current.preempt_count += 1;
        }
    }

    fn preempt_enable(&mut self) {
        if let Some(current) = self.current() {
            current.preempt_count -= 1;
        }
    }

    fn switch_to(&mut self, next: usize) {
        if self.current == next {
            return;
        }

        crate::trace!("Switching to task id {}", next);

        let last = self.tasks.get(self.current).expect("last");
        self.current = next;
        let next = self.tasks.get(next).expect("next");

        cpu_switch_to(last, next);
    }
}

impl<const C: usize> Scheduler<C> {
    pub const fn new() -> Self {
        Self {
            inner: IRQSafeNullLock::new(SchedulerInner::new()),
        }
    }

    pub fn init(&self) {
        self.inner.map_locked(|inner| inner.init());
    }

    pub fn register_new_waiting_task(&self, task: WrappedPointer<Task>) {
        self.inner.map_locked(|inner| inner.push_task(task))
    }

    fn preempt_enable(&self) {
        self.inner.map_locked(|inner| inner.preempt_enable());
    }

    fn preempt_disable(&self) {
        self.inner.map_locked(|inner| inner.preempt_disable());
    }

    pub(crate) fn schedule(&self) {
        self.inner.map_locked(|inner| {
            let current = if let Some(current) = inner.current() {
                current
            } else {
                return;
            };
            current.counter = current.counter.saturating_sub(1);
            if current.counter > 0 || current.preempt_count > 0 {
                return;
            }
            current.counter = 0;
            unmask_irq();
            inner.schedule();
            mask_irq();
        })
    }
}

impl<const C: usize> TickCallbackHandler for Scheduler<C> {
    fn handle(&self) {
        self.schedule()
    }
}

fn cpu_switch_to(last: &Task, new: &Task) {
    unsafe { Task::cpu_switch_to(last, new) }
}

pub unsafe fn spawn_process(f: u64, arg: u64) -> Result<(), &'static str> {
    SCHEDULER.preempt_disable();
    let page = next_free_page()?;
    let mut task: WrappedPointer<Task> = WrappedPointer::new(page.addr());

    let child_regs = pt_regs(&task);
    (child_regs.addr() as *mut PtRegs).write_bytes(0, 1);
    (task.addr() as *mut CpuContext).write_bytes(0, 1); // This works on behalf of CpuContext being first field

    task.context.x19 = f;
    task.context.x20 = arg;

    task.priority = 10;
    task.state = TaskState::Running;
    task.counter = task.priority;
    task.preempt_count = 1;

    task.context.pc = return_from_fork.get() as u64;
    task.context.sp = child_regs.addr() as u64;

    SCHEDULER.register_new_waiting_task(task);
    SCHEDULER.preempt_enable();

    Ok(())
}

unsafe fn pt_regs(task: &WrappedPointer<Task>) -> WrappedPointer<PtRegs> {
    WrappedPointer::new(task.addr() + 4096 - size_of::<PtRegs>())
}

#[no_mangle]
fn schedule_tail() {
    SCHEDULER.preempt_enable()
}
