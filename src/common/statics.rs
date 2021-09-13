use crate::common::state::KernelInitManager;
pub use crate::{arch::arch_impl::statics::*, bsp::device::statics::*};

pub static STATE_MANAGER: KernelInitManager = KernelInitManager::new();
pub use crate::common::memory::mmu::mapping::KERNEL_MAPPING_RECORD;
