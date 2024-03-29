use bitaccess::bitaccess;

#[bitaccess(
    base_type = u64,
    kind = write_only,
    write_via = r#"unsafe { core::arch::asm!("msr cntvoff_el2, {}", in(reg) value, options(nostack, nomem)); }"#
)]
pub enum CntvoffEl2 {}
