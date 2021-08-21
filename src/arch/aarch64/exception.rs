use crate::common::exception::PrivilegeLevel;
use crate::arch::arch_impl::cpu::registers::current_el;
use crate::arch::aarch64::cpu::registers::ExceptionLevel;

pub fn current_privilege_level() -> PrivilegeLevel {
    let el = unsafe { current_el() };
    match el {
        ExceptionLevel::EL0 => PrivilegeLevel::User,
        ExceptionLevel::EL1 => PrivilegeLevel::Kernel,
        ExceptionLevel::EL2 => PrivilegeLevel::Hypervisor,
        ExceptionLevel::EL3 => PrivilegeLevel::Firmware,
    }
}
