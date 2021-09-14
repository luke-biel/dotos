pub mod driver;
pub mod exception;
pub mod memory;
pub mod scheduler;
pub mod serial_console;
pub mod state;
pub mod statics;
pub mod sync;
pub mod task;
pub mod time_manager;

pub const fn align_down<const SHIFT: usize>(value: usize) -> usize {
    value & !((1 << SHIFT) - 1)
}
