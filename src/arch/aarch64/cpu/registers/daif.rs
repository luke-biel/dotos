use bitaccess::{bitaccess, FieldAccess};
use derive_more::Display;

#[bitaccess(
    base_type = u64,
    kind = read_write,
    read_via = r#"unsafe { asm!("mrs {}, daif", out(reg) value, options(nostack, nomem)); }"#,
    write_via = r#"unsafe { asm!("msr daif, {}", in(reg) value, options(nostack, nomem)) }"#
)]
pub enum Daif {
    #[bit(9)]
    #[variants(Mask)]
    Debug,
    #[bit(8)]
    #[variants(Mask)]
    SError,
    #[bit(7)]
    #[variants(Mask)]
    IRQ,
    #[bit(6)]
    #[variants(Mask)]
    FIQ,
}

#[derive(Display, FieldAccess)]
#[field_access(u64)]
pub enum Mask {
    Masked,
    Unmasked,
}
