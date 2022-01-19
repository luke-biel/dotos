use bitaccess::bitaccess;

#[bitaccess(
    base_type = u64,
    kind = write_only,
    write_via = r#"unsafe { core::arch::asm!("msr spsr_el2, {}", in(reg) value, options(nostack, nomem)); }"#
)]
pub enum SpsrEl2 {}
