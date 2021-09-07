use crate::{
    arch::arch_impl::{
        memory::mmu::{translation_table::KernelTranslationTable, Aarch64MemoryManagementUnit},
        time::GenericTimer,
    },
    common::sync::InitStateLock,
};

pub static TIMER: GenericTimer = GenericTimer;
pub static MMU: Aarch64MemoryManagementUnit = Aarch64MemoryManagementUnit;
pub static KERNEL_TABLES: InitStateLock<KernelTranslationTable> =
    unsafe { super::memory::mmu::KERNEL_TABLES };
