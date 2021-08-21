use core::ops::RangeInclusive;

use crate::{
    bsp::{device::memory::map::END, rpi3::memory::map::mmio},
    common::memory::mmu::{
        AccessPermissions,
        AddressSpace,
        Attributes,
        Execute,
        KernelVirtualLayout,
        MemoryAttributes,
        Translation,
        TranslationDescriptor,
    },
};

pub type KernelAddrSpace = AddressSpace<{ END + 1 }>;

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
