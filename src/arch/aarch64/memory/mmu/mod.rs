use crate::arch::aarch64::memory::mmu::translation_table::KernelTranslationTable;
use crate::common::memory::interface::MMUInterface;
use crate::common::memory::{AddressSpace, TranslationGranule};
use cortex_a::asm::{barrier};
use cortex_a::registers::*;
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use crate::bsp::raspberry_pi_3::memory::mmu::KernelAddressSpace;

pub mod mair;
pub mod translation_table;

pub struct MemoryManagementUnit;

pub type Granule64KB = TranslationGranule<{ 64 * 1024 }>;
pub type Granule512MB = TranslationGranule<{ 512 * 1024 * 1024 }>;

pub static mut KERNEL_TABLES: KernelTranslationTable = KernelTranslationTable::new();
pub static MMU: MemoryManagementUnit = MemoryManagementUnit;

impl<const SIZE: usize> AddressSpace<SIZE> {
    pub const fn arch_address_space_size_sanity_check() {
        assert!(SIZE % Granule512MB::SIZE == 0);
        assert!(SIZE <= 1 << 48);
    }
}

impl MemoryManagementUnit {
    fn setup_mair(&self) {
        MAIR_EL1.write(
            MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc
                + MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc
                + MAIR_EL1::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck,
        );
    }

    fn configure_translation_table(&self) {
        let t0sz = (64 - KernelAddressSpace::SHIFT) as u64;

        TCR_EL1.write(
            TCR_EL1::TBI0::Used
                + TCR_EL1::IPS::Bits_40
                + TCR_EL1::TG0::KiB_64
                + TCR_EL1::SH0::Inner
                + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
                + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
                + TCR_EL1::EPD0::EnableTTBR0Walks
                + TCR_EL1::A1::TTBR0
                + TCR_EL1::T0SZ.val(t0sz)
                + TCR_EL1::EPD1::DisableTTBR1Walks,
        );
    }
}

impl MMUInterface for MemoryManagementUnit {
    unsafe fn enable_mmu_and_caching(&self) -> Result<(), &'static str> {
        if self.is_enabled() {
            return Err("aleardy enabled");
        }

        if !ID_AA64MMFR0_EL1.matches_all(ID_AA64MMFR0_EL1::TGran64::Supported) {
            return Err("Translation granule not supported by hardware");
        }

        self.setup_mair();

        KERNEL_TABLES.populate_table_entries()?;

        TTBR0_EL1.set_baddr(KERNEL_TABLES.base_paddr());

        self.configure_translation_table();

        barrier::isb(barrier::SY);

        SCTLR_EL1.modify(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);

        barrier::isb(barrier::SY);

        Ok(())
    }

    fn is_enabled(&self) -> bool {
        SCTLR_EL1.matches_all(SCTLR_EL1::M::Enable)
    }
}
