#[cfg(target_arch = "aarch64")]
pub use aarch64 as arch_impl;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
