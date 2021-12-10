use crate::{
    arch::arch_impl::{memory::mmu::Aarch64MemoryManagementUnit, time::GenericTimer},
    common::sync::IRQSafeNullLock,
};

pub static CLOCK_TIMER: IRQSafeNullLock<GenericTimer> = IRQSafeNullLock::new(GenericTimer);
pub static MMU: Aarch64MemoryManagementUnit = Aarch64MemoryManagementUnit;
pub use super::memory::mmu::KERNEL_TABLES;
