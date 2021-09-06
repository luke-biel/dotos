use crate::common::state::KernelInitManager;
pub use crate::{arch::arch_impl::statics::*, bsp::device::statics::*};

pub static STATE_MANAGER: KernelInitManager = KernelInitManager::new();
