use core::{fmt, fmt::Formatter};

pub mod asynchronous;

#[derive(PartialEq)]
pub enum PrivilegeLevel {
    User,
    Kernel,
    Hypervisor,
    Firmware,
}

impl fmt::Display for PrivilegeLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PrivilegeLevel::User => "user",
            PrivilegeLevel::Kernel => "kernel",
            PrivilegeLevel::Hypervisor => "hypervisor",
            PrivilegeLevel::Firmware => "unknown",
        }
        .fmt(f)
    }
}
