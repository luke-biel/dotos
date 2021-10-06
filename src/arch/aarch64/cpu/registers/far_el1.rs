use bitaccess::bitaccess;

#[bitaccess(
    base_type = u64,
    kind = read_only,
    read_via = r#"unsafe { asm!("mrs {}, far_el1", out(reg) value, options(nostack, nomem)); }"#
)]
pub enum FarEl1 {}
