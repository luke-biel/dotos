use bitaccess::{bitaccess, ReadBits};

#[bitaccess(
    base_type = u64,
    kind = read_only,
    read_via = r#"unsafe { core::arch::asm!("mrs {}, mpidr_el1", out(reg) value, options(nomem, nostack)); }"#
)]
pub enum MpidrEl1 {
    #[bits(0..2)]
    CoreId,
}

pub unsafe fn core_id_el1() -> u64 {
    MpidrEl1.read(MpidrEl1::CoreId).value()
}
