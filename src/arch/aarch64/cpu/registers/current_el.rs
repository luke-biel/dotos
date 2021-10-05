use bitaccess::{bitaccess, FieldAccess};

#[derive(FieldAccess, PartialEq)]
#[field_access(u64)]
pub enum ExceptionLevel {
    EL0 = 0b0000,
    EL1 = 0b0100,
    EL2 = 0b1000,
    EL3 = 0b1100,
}

#[bitaccess(
    base_type = u64,
    kind = read_only,
    read_via = r#"unsafe { asm!("mrs {}, currentel", out(reg) value, options(nomem, nostack)) }"#
)]
pub enum CurrentEl {
    #[bits(0..4)]
    #[variants(ExceptionLevel)]
    Status,
}
