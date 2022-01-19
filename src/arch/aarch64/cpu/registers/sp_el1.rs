use bitaccess::bitaccess;

#[bitaccess(
    base_type = u64,
    kind = write_only,
    write_via = r#"unsafe { core::arch::asm!("msr sp_el1, {}", in(reg) value, options(nostack, nomem)); }"#
)]
pub enum SpEl1 {}
