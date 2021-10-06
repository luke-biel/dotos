use crate::{
    arch::{
        aarch64::memory::mmu::Granule64KB,
        arch_impl::cpu::exception::{
            asynchronous::{mask_irq, unmask_irq},
            return_from_fork,
        },
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
                        task.counter = (task.counter >> 1) + task.priority
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

pub unsafe fn spawn_process(f: fn(), priority: u64) -> Result<(), &'static str> {
    SCHEDULER.preempt_disable();
    let page = next_free_page()?;
    let mut task: WrappedPointer<Task> = WrappedPointer::new(page.addr());

    task.priority = priority;
    task.state = TaskState::Running;
    task.counter = priority;
    task.preempt_count = 1;

    task.context.registers[0] = f as usize as u64; // x19
    task.context.pc = return_from_fork.get() as u64;
    task.context.sp = (page.addr() + Granule64KB::SIZE) as u64;

    SCHEDULER.register_new_waiting_task(task);
    SCHEDULER.preempt_enable();

    Ok(())
}

#[no_mangle]
fn schedule_tail() {
    SCHEDULER.preempt_enable()
}
