use core::intrinsics::unlikely;

use crate::{
    arch::arch_impl::memory::mmu::translation_table::KernelTranslationTable,
    bsp::device::memory::mmu::KernelAddrSpace,
    common::{
        memory::{
            mmu::{AddressSpace, MemoryManagementUnit, TranslationGranule},
            Address,
            Physical,
        },
        sync::InitStateLock,
    },
};

pub mod mair;
pub mod translation_table;

pub struct Aarch64MemoryManagementUnit;

pub type Granule512MB = TranslationGranule<{ 512 * 1024 * 1024 }>;
pub type Granule64KB = TranslationGranule<{ 64 * 1024 }>;

pub static KERNEL_TABLES: InitStateLock<KernelTranslationTable> =
    InitStateLock::new(KernelTranslationTable::new());

impl<const SIZE: usize> AddressSpace<SIZE> {
    pub const fn arch_address_space_size_sanity_checks() {
        assert!((SIZE % Granule512MB::SIZE) == 0);
        assert!(SIZE <= (1 << 48));
    }
}

impl Aarch64MemoryManagementUnit {
    fn setup_mair(&self) {
        let mair_el1: u64 = 0b1111_1111_0000_0100;
        unsafe { asm!("msr mair_el1, {}", in(reg) mair_el1, options(nostack, nomem)) };
    }

    fn configure_translation_control(&self) {
        let t0sz = (64 - KernelAddrSpace::SHIFT) as u64;

        let tcr_el1: u64 = (0b010 << 32) // IPS Bits40
            // + (0 << 37) // TBI0 Used
            + (0b01 << 14) // TG0 64KB
            + (0b11 << 12) // SH0 Inner
            + (0b01 << 10) // ORGN0 WB_RA_WA_C
            + (0b01 << 8) // IRGN0 WB_RA_WA_C
            // + (0 << 7) // EPD0 EnableTTBR0 Walks
            // + (0 << 22) // A1 TTBR0
            + (t0sz & 0b11_1111) // T0SZ
            + (1 << 23); // EPD1 DisableTTBR1 Walks
        unsafe { asm!("msr tcr_el1, {}", in(reg) tcr_el1, options(nostack, nomem)) };
    }
}

impl MemoryManagementUnit for Aarch64MemoryManagementUnit {
    unsafe fn enable_mmu_and_caching(
        &self,
        translation_table_base_addr: Address<Physical>,
    ) -> Result<(), &'static str> {
        if unlikely(self.is_enabled()) {
            return Err("MMU is already enabled");
        }

        let granule_size: u64;
        asm!("mrs {}, id_aa64mmfr0_el1", out(reg) granule_size, options(nostack, nomem));
        if unlikely((granule_size & (0b1111 << 24)) != 0) {
            return Err("translation granule size not supported by hw");
        }

        self.setup_mair();

        let baddr: u64 = translation_table_base_addr.addr() as u64;
        asm!("msr ttbr0_el1, {}", in(reg) baddr, options(nostack, nomem));

        self.configure_translation_control();

        asm!("isb sy");

        let mut sctlr_el1: u64;
        asm!("mrs {}, sctlr_el1", out(reg) sctlr_el1, options(nostack, nomem));
        sctlr_el1 &= (1 << 12) + (1 << 2) + 1;
        asm!("msr sctlr_el1, {}", in(reg) sctlr_el1, options(nostack, nomem));

        asm!("isb sy");

        Ok(())
    }

    fn is_enabled(&self) -> bool {
        let sctlr_el1: u64;
        unsafe { asm!("mrs {}, sctlr_el1", out(reg) sctlr_el1, options(nostack, nomem)) };
        sctlr_el1 & 1 > 0
    }
}
