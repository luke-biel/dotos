pub mod driver;
pub mod exception;
pub mod memory;
pub mod serial_console;
pub mod state;
pub mod statics;
pub mod sync;
pub mod time_manager;

pub const fn align_down<const T: usize>(value: usize) -> usize {
    value & !((1 << T) - 1)
}
