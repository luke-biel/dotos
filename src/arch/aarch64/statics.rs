use crate::arch::arch_impl::{memory::mmu::Aarch64MemoryManagementUnit, time::GenericTimer};

pub static TIMER: GenericTimer = GenericTimer;
pub static MMU: Aarch64MemoryManagementUnit = Aarch64MemoryManagementUnit;
