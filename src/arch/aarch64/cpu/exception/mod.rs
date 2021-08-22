use core::{cell::UnsafeCell, fmt, fmt::Formatter};

pub mod asynchronous;
mod handlers;

// TODO: look whether I can replace this with rust code
global_asm!(include_str!("exception.s"));

extern "Rust" {
    static __exv_start: UnsafeCell<()>;
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
        write!(f, "registers:\n")?;
        for i in 0..30 {
            write!(f, "    x{} = 0x{:0x}\n", i, self.registers[i])?;
        }
        write!(f, "link_register: 0x{:0x}\n", self.link_register)?;
        write!(f, "elr_el1: 0x{:0x}\n", self.elr_el1)?;
        write!(f, "spsr_el1: 0x{:0x}\n", self.spsr_el1)
    }
}

pub unsafe fn init_exception_handling() {
    let vbar_el1: u64 = __exv_start.get() as u64;
    asm!("msr vbar_el1, {}", in(reg) vbar_el1, options(nostack, nomem));
    asm!("isb sy");
}
