use core::ops::RangeInclusive;

use crate::{
    arch::arch_impl::memory::mmu::translation_table::FixedSizeTranslationTable,
    bsp::{device::memory::map::END, rpi3::memory::map::mmio},
    common::{
        align_down,
        memory::{
            mmu::{
                descriptors::{
                    AccessPermissions,
                    Attributes,
                    Execute,
                    MemoryAttributes,
                    Page,
                    Translation,
                    TranslationDescriptor,
                },
                AddressSpace,
                AssociatedTranslationTable,
                KernelVirtualLayout,
                TranslationGranule,
            },
            Physical,
        },
        sync::InitStateLock,
    },
};

pub type KernelGranule = TranslationGranule<{ 64 * 1024 }>;
pub type KernelAddrSpace = AddressSpace<{ 8 * 1024 * 1024 * 1024 }>;

const NUM_MEM_RANGES: usize = 3;

pub static KERNEL_VIRTUAL_LAYOUT: KernelVirtualLayout<NUM_MEM_RANGES> = KernelVirtualLayout::new(
    END,
    [
        TranslationDescriptor {
            name: "Kernel code and RO data",
            vrange: rx_range_inclusive,
            prange_translation: Translation::Id,
            attributes: Attributes {
                memory: MemoryAttributes::CacheableDRAM,
                access: AccessPermissions::RX,
                execute: Execute::Always,
            },
        },
        TranslationDescriptor {
            name: "Remapped Device MMIO",
            vrange: remapped_mmio_range_inclusive,
            prange_translation: Translation::Offset(mmio::START + 0x20_0000),
            attributes: Attributes {
                memory: MemoryAttributes::Device,
                access: AccessPermissions::RW,
                execute: Execute::Never,
            },
        },
        TranslationDescriptor {
            name: "Device MMIO",
            vrange: mmio_range_inclusive,
            prange_translation: Translation::Id,
            attributes: Attributes {
                memory: MemoryAttributes::Device,
                access: AccessPermissions::RW,
                execute: Execute::Never,
            },
        },
    ],
);

fn rx_range_inclusive() -> RangeInclusive<usize> {
    RangeInclusive::new(super::rx_start(), super::rx_ende() - 1)
}

fn remapped_mmio_range_inclusive() -> RangeInclusive<usize> {
    RangeInclusive::new(0x1FFF_0000, 0x1FFF_FFFF)
}

fn mmio_range_inclusive() -> RangeInclusive<usize> {
    RangeInclusive::new(mmio::START, mmio::END)
}

pub fn add_space_end_page() -> *const Page<Physical> {
    align_down::<KernelGranule::SIZE>(super::map::END) as *const _
}
