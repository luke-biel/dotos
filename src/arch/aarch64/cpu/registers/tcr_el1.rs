use bitaccess::{bitaccess, FieldAccess};
use derive_more::Display;

#[bitaccess(
    base_type = u64,
    kind = read_write,
    read_via = r#"unsafe { asm!("mrs {}, tcr_el1", out(reg) value, options(nostack, nomem)); }"#,
    write_via = r#"unsafe { asm!("msr tcr_el1, {}", in(reg) value, options(nostack, nomem)); }"#
)]
pub enum TcrEl1 {
    #[bit(59)]
    #[variants(Unset => 0, Set => 1)]
    DS,
    #[bit(58)]
    #[variants(Unset => 0, Set => 1)]
    TCMA1,
    #[bit(57)]
    #[variants(Unset => 0, Set => 1)]
    TCMA0,
    #[bit(56)]
    #[variants(Unset => 0, Set => 1)]
    E0PD1,
    #[bit(55)]
    #[variants(Unset => 0, Set => 1)]
    E0PD0,
    #[bit(54)]
    #[variants(Unset => 0, Set => 1)]
    NFD1,
    #[bit(53)]
    #[variants(Unset => 0, Set => 1)]
    NFD0,
    #[bit(52)]
    #[variants(Unset => 0, Set => 1)]
    TBID1,
    #[bit(51)]
    #[variants(Unset => 0, Set => 1)]
    TBID0,
    #[bit(50)]
    #[variants(Unset => 0, Set => 1)]
    HWU162,
    #[bit(49)]
    #[variants(Unset => 0, Set => 1)]
    HWU161,
    #[bit(48)]
    #[variants(Unset => 0, Set => 1)]
    HWU160,
    #[bit(47)]
    #[variants(Unset => 0, Set => 1)]
    HWU159,
    #[bit(46)]
    #[variants(Unset => 0, Set => 1)]
    HWU062,
    #[bit(45)]
    #[variants(Unset => 0, Set => 1)]
    HWU061,
    #[bit(44)]
    #[variants(Unset => 0, Set => 1)]
    HWU060,
    #[bit(43)]
    #[variants(Unset => 0, Set => 1)]
    HWU059,
    #[bit(42)]
    #[variants(Unset => 0, Set => 1)]
    HPD1,
    #[bit(41)]
    #[variants(Unset => 0, Set => 1)]
    HPD0,
    #[bit(40)]
    #[variants(Unset => 0, Set => 1)]
    HD,
    #[bit(39)]
    #[variants(Unset => 0, Set => 1)]
    HA,
    #[bit(38)]
    #[variants(Unset => 0, Set => 1)]
    TBI1,
    #[bit(37)]
    #[variants(Unset => 0, Set => 1)]
    TBI0,
    #[bit(36)]
    #[variants(Unset => 0, Set => 1)]
    AS,
    #[bits(32..35)]
    #[variants(IPSVariants)]
    IPS,
    #[bits(30..32)]
    #[variants(GranuleSize1)]
    TG1,
    #[bits(28..30)]
    #[variants(Shareability)]
    SH1,
    #[bits(26..28)]
    #[variants(OuterCacheability)]
    ORGN1,
    #[bits(24..26)]
    #[variants(InnerCacheability)]
    IRGN1,
    #[bit(23)]
    #[variants(Enable => 0, Disable => 1)]
    EPD1,
    #[bit(22)]
    #[variants(TTBR0 => 0, TTBR1 => 1)]
    A1,
    #[bits(16..22)]
    T1SZ,
    #[bits(14..16)]
    #[variants(GranuleSize0)]
    TG0,
    #[bits(12..14)]
    #[variants(Shareability)]
    SH0,
    #[bits(10..12)]
    #[variants(OuterCacheability)]
    ORGN0,
    #[bits(8..10)]
    #[variants(InnerCacheability)]
    IRGN0,
    #[bit(7)]
    #[variants(Enable => 0, Disable => 1)]
    EPD0,
    #[bits(0..6)]
    T0SZ,
}

#[derive(FieldAccess, Display)]
#[field_access(u64)]
pub enum IPSVariants {
    Bits32 = 0b000,
    Bits36 = 0b001,
    Bits40 = 0b010,
    Bits42 = 0b011,
    Bits44 = 0b100,
    Bits48 = 0b101,
    Bits52 = 0b110,
}

#[derive(FieldAccess, Display)]
#[field_access(u64)]
pub enum GranuleSize1 {
    KB16 = 0b01,
    KB4 = 0b10,
    KB64 = 0b11,
}

#[derive(FieldAccess, Display)]
#[field_access(u64)]
pub enum GranuleSize0 {
    KB4 = 0b00,
    KB64 = 0b01,
    KB16 = 0b10,
}

#[derive(FieldAccess, Display)]
#[field_access(u64)]
pub enum Shareability {
    None = 0b00,
    Outer = 0b10,
    Inner = 0b11,
}

#[allow(non_camel_case_types)]
#[derive(FieldAccess, Display)]
#[field_access(u64)]
pub enum OuterCacheability {
    NonCacheable = 0b00,
    WriteBack_ReadAlloc_WriteAlloc = 0b01,
    WriteThrough_ReadAlloc_NoWriteAlloc = 0b10,
    WriteBack_ReadAlloc_NoWriteAlloc = 0b11,
}

#[allow(non_camel_case_types)]
#[derive(FieldAccess, Display)]
#[field_access(u64)]
pub enum InnerCacheability {
    NonCacheable = 0b00,
    WriteBack_ReadAlloc_WriteAlloc = 0b01,
    WriteThrough_ReadAlloc_NoWriteAlloc = 0b10,
    WriteBack_ReadAlloc_NoWriteAlloc = 0b11,
}
