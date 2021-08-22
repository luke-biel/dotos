use core::fmt;
use core::fmt::Formatter;

pub mod asynchronous;
mod handlers;

// TODO: look whether I can replace this with rust code
global_asm!(include_str!("exception.s"));

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
        todo!()
    }
}
