pub use crate::common::sync::{
    init_state_lock::{InitStateLock, ReadWriteLock},
    irq_safe_null_lock::{IRQSafeNullLock, Mutex},
};

mod init_state_lock;
mod irq_safe_null_lock;
