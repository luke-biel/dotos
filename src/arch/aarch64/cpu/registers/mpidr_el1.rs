use bitaccess::bitaccess;

#[bitaccess(
    base_type = u64,
    kind = read_only,
    read_via = r#"unsafe { asm!("mrs {}, mpidr_el1", out(reg) value, options(nomem, nostack)); }"#
)]
pub enum MpidrEl1 {
    #[bits(0..2)]
    CoreId,
}
