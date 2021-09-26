use bitaccess::{bitaccess, FieldAccess};
use core::fmt;
use derive_more::Display;

// register_bitfields! {
//     u64,
//     ESR_EL1 [
//         EC OFFSET(26) NUMBITS(6) [],
//         IL OFFSET(25) NUMBITS(1) [],
//         ISS OFFSET(0) NUMBITS(25) []
//     ],
//
//     ISSDataAbort [
//         ISV OFFSET(24) NUMBITS(1) [
//             NoValid = 0,
//             Valid = 1
//         ],
//         SAS OFFSET(22) NUMBITS(2) [
//             Byte = 0,
//             Halfword = 1,
//             Word = 2,
//             Doubleword = 3
//         ],
//         SSE OFFSET(21) NUMBITS(1) [],
//         SRT OFFSET(16) NUMBITS(5) [],
//         SF OFFSET(15) NUMBITS(1) [],
//         AR OFFSET(14) NUMBITS(1) [],
//         VNCR OFFSET(13) NUMBITS(1) [],
//         SET OFFSET(11) NUMBITS(1) [
//             Recoverable = 0,
//             Uncontainable = 2,
//             Restartable = 3,
//         ],
//         FnV OFFSET(10) NUMBITS(1) [
//             FarValid = 0,
//             FarInvalid = 1,
//         ],
//         EA OFFSET(9) NUMBITS(1) [],
//         CM OFFSET(8) NUMBITS(1) [],
//         S1PTW OFFSET(7) NUMBITS(1) [],
//         WnR OFFSET(6) NUMBITS(1) [
//             Reading = 0,
//             Writing = 1,
//         ],
//         DFSC OFFSET(0) NUMBITS(6) [
//             AddressSizeLevel0 = 0x0,
//             AddressSizeLevel1 = 0x1,
//             AddressSizeLevel2 = 0x2,
//             AddressSizeLevel3 = 0x3,
//             TranslationFaultLevel0 = 0x4,
//             TranslationFaultLevel1 = 0x5,
//             TranslationFaultLevel2 = 0x6,
//             TranslationFaultLevel3 = 0x7,
//             AccessFlagFaultLevel0 = 0x8,
//             AccessFlagFaultLevel1 = 0x9,
//             AccessFlagFaultLevel2 = 0xa,
//             AccessFlagFaultLevel3 = 0xb,
//             PermissionFaultLevel0 = 0xc,
//             PermissionFaultLevel1 = 0xd,
//             PermissionFaultLevel2 = 0xe,
//             PermissionFaultLevel3 = 0xf,
//
//             SynchronousExternalAbortNoTT = 0x10,
//             SynchronousTagCheckFault = 0x11,
//             SynchronousExternalAbortLevelM1 = 0x13,
//             SynchronousExternalAbortLevel0 = 0x14,
//             SynchronousExternalAbortLevel1 = 0x15,
//             SynchronousExternalAbortLevel2 = 0x16,
//             SynchronousExternalAbortLevel3 = 0x17,
//
//             SynchronousParityOrECCNoTT = 0x18,
//             SynchronousParityOrECCLevelM1 = 0x1b,
//             SynchronousParityOrECCLevel0 = 0x1c,
//             SynchronousParityOrECCLevel1 = 0x1d,
//             SynchronousParityOrECCLevel2 = 0x1e,
//             SynchronousParityOrECCLevel3 = 0x1f,
//
//             AlignmentFault = 0x21,
//             AddressSizeLevelM1 = 0x29,
//             TranslationFaultLevelM1 = 0x2b,
//
//             TLBConflictAbort = 0x30,
//             UnsupportedAtomicHardwareUpdate = 0x31,
//             ImplementationDefinedLockdown = 0x34,
//             ImplementationDefinedExclusive = 0x35,
//         ]
//     ]
// }

#[bitaccess(
    base_type = u64,
    kind = read_only,
    read_via = r#"unsafe { asm!("mrs {}, esr_el1", out(reg) value, options(nostack, nomem)); }"#
)]
pub enum EsrEl1 {
    #[bits(26..32)]
    EC,
    #[bit(25)]
    IL,
    #[bits(0..25)]
    ISS,
}

#[bitaccess(base_type = u64, kind = read_only)]
pub enum ISSDataAbort {
    #[bit(24)]                               ISV,
    #[bits(22..24)]                          SAS,
    #[bit(21)]                               SSE,
    #[bits(16..21)]                          SRT,
    #[bit(15)]                               SF,
    #[bit(14)]                               AR,
    #[bit(13)]                               VNCR,
    #[bits(11..13)]                          SET,
    #[bit(10)]                               FnV,
    #[bit(9)]                                EA,
    #[bit(8)]                                CM,
    #[bit(7)]                                S1PTW,
    #[bit(6)]                                WnR,
    #[bits(0..6)] #[variants(DfscVariants)]  DFSC,
}

#[derive(FieldAccess, Display)]
#[field_access(u64)]
pub enum DfscVariants {
    AddressSizeLevel0 = 0x0,
    AddressSizeLevel1 = 0x1,
    AddressSizeLevel2 = 0x2,
    AddressSizeLevel3 = 0x3,

    TranslationFaultLevel0 = 0x4,
    TranslationFaultLevel1 = 0x5,
    TranslationFaultLevel2 = 0x6,
    TranslationFaultLevel3 = 0x7,

    AccessFlagFaultLevel0 = 0x8,
    AccessFlagFaultLevel1 = 0x9,
    AccessFlagFaultLevel2 = 0xa,
    AccessFlagFaultLevel3 = 0xb,

    PermissionFaultLevel0 = 0xc,
    PermissionFaultLevel1 = 0xd,
    PermissionFaultLevel2 = 0xe,
    PermissionFaultLevel3 = 0xf,

    SynchronousExternalAbortNoTT = 0x10,
    SynchronousTagCheckFault = 0x11,
    SynchronousExternalAbortLevelM1 = 0x13,
    SynchronousExternalAbortLevel0 = 0x14,
    SynchronousExternalAbortLevel1 = 0x15,
    SynchronousExternalAbortLevel2 = 0x16,
    SynchronousExternalAbortLevel3 = 0x17,
    SynchronousParityOrECCNoTT = 0x18,
    SynchronousParityOrECCLevelM1 = 0x1b,
    SynchronousParityOrECCLevel0 = 0x1c,
    SynchronousParityOrECCLevel1 = 0x1d,
    SynchronousParityOrECCLevel2 = 0x1e,
    SynchronousParityOrECCLevel3 = 0x1f,

    AlignmentFault = 0x21,
    AddressSizeLevelM1 = 0x29,
    TranslationFaultLevelM1 = 0x2b,
    TLBConflictAbort = 0x30,
    UnsupportedAtomicHardwareUpdate = 0x31,

    ImplementationDefinedLockdown = 0x34,
    ImplementationDefinedExclusive = 0x35,
}

impl fmt::Display for EsrEl1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = Self::fetch();
        write!(f, "EC: ")?;
        let is_iss_data_abort = match val.read(EsrEl1::EC).value() {
            0b10_0100 => { write!(f, "`DataAbortLowerEL`")?; true },
            0b10_0101 => { write!(f, "`DataAbortCurrentEL`")?; true }
            val => { write!(f, "{}", val)?; false }
        };

        write!(f, "\nIL: {}\nISS: \n", val.read(EsrEl1::IL).value())?;

        if is_iss_data_abort {
            let iss_data_abort = ISSDataAbort::from_value(val.read(EsrEl1::ISS).value());
            write!(f, "{}", iss_data_abort)?;
        } else {
            write!(f, "{}", val.read(EsrEl1::ISS).value())?;
        }

        Ok(())
    }
}

impl fmt::Display for ISSDataAbort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ISV: {},", self.read(ISSDataAbort::ISV).value())?;
        writeln!(f, "SAS: {},", self.read(ISSDataAbort::SAS).value())?;
        writeln!(f, "SSE: {},", self.read(ISSDataAbort::SSE).value())?;
        writeln!(f, "SRT: {},", self.read(ISSDataAbort::SRT).value())?;
        writeln!(f, "SF: {},", self.read(ISSDataAbort::SF).value())?;
        writeln!(f, "AR: {},", self.read(ISSDataAbort::AR).value())?;
        writeln!(f, "VNCR: {},", self.read(ISSDataAbort::VNCR).value())?;
        writeln!(f, "SET: {},", self.read(ISSDataAbort::SET).value())?;
        writeln!(f, "FnV: {},", self.read(ISSDataAbort::FnV).value())?;
        writeln!(f, "EA: {},", self.read(ISSDataAbort::EA).value())?;
        writeln!(f, "CM: {},", self.read(ISSDataAbort::CM).value())?;
        writeln!(f, "S1PTW: {},", self.read(ISSDataAbort::S1PTW).value())?;
        writeln!(f, "WnR: {},", self.read(ISSDataAbort::WnR).value())?;
        writeln!(f, "DFSC: {},", self.read(ISSDataAbort::DFSC).variant())
    }
}
