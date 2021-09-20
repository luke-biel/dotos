use core::{cell::UnsafeCell, fmt, fmt::Formatter};

use crate::{
    arch::aarch64::cpu::registers::{current_el, ExceptionLevel},
    common::exception::PrivilegeLevel,
};

pub mod asynchronous;
mod handlers;

// TODO: look whether I can replace this with rust code
global_asm!(include_str!("exception.s"));

extern "Rust" {
    pub static return_from_fork: UnsafeCell<()>;
}

#[derive(Debug)]
#[repr(C)]
pub struct ExceptionContext {
    registers: [u64; 30],
    link_register: u64,
    elr_el1: u64,
    spsr_el1: u64,
}

impl fmt::Display for ExceptionContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "registers:")?;
        for i in 0..30 {
            writeln!(f, "    x{} = 0x{:0x}", i, self.registers[i])?;
        }
        writeln!(f, "link_register: 0x{:0x}", self.link_register)?;
        writeln!(f, "elr_el1: 0x{:0x}", self.elr_el1)?;
        writeln!(f, "spsr_el1: 0x{:0x}", self.spsr_el1)
    }
}

#[inline(always)]
pub unsafe fn init_exception_handling() {
    extern "Rust" {
        static __exception_vector_addr: UnsafeCell<()>;
    }

    let vbar_el1: u64 = __exception_vector_addr.get() as u64;
    asm!("msr vbar_el1, {}", in(reg) vbar_el1);
    asm!("isb sy");
}

pub fn current_privilege_level() -> PrivilegeLevel {
    let el = unsafe { current_el() };
    match el {
        ExceptionLevel::EL0 => PrivilegeLevel::User,
        ExceptionLevel::EL1 => PrivilegeLevel::Kernel,
        ExceptionLevel::EL2 => PrivilegeLevel::Hypervisor,
        ExceptionLevel::EL3 => PrivilegeLevel::Firmware,
    }
}
