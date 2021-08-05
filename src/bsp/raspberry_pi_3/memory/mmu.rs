use crate::common::memory::AddressSpace;
use crate::bsp::raspberry_pi_3::memory::{ENDI, MMIO_BASE, rx_start, rx_end_exclusive, MMIO_ENDI};
use crate::common::memory::mmu::{KernelVirtualLayout, TranslationDescriptor, Attributes, Translation, MemAttributes, Access, ExecutionPolicy};
use core::ops::RangeInclusive;

pub type KernelAddressSpace = AddressSpace<{ ENDI + 1 }>;

const NUM_MEM_RANGES: usize = 3;

pub static LAYOUT: KernelVirtualLayout<NUM_MEM_RANGES> = KernelVirtualLayout::new(ENDI, [
    TranslationDescriptor {
        name: "Kernel code and RO data",
        virtual_range: rx_range_inclusive,
        translation: Translation::Identity,
        attributes: Attributes {
            mem_attributes: MemAttributes::Device,
            access_permissions: Access::ReadWrite,
            execute: ExecutionPolicy::Never
        }
    },
    TranslationDescriptor {
        name: "Remapped device MMIO",
        virtual_range: remapped_mmio_range_inclusive,
        translation: Translation::Offset(MMIO_BASE + 0x20_0000),
        attributes: Attributes {
            mem_attributes: MemAttributes::Device,
            access_permissions: Access::ReadWrite,
            execute: ExecutionPolicy::Never
        }
    },
    TranslationDescriptor {
        name: "Device MMIO",
        virtual_range: mmio_range_inclusive,
        translation: Translation::Identity,
        attributes: Attributes {
            mem_attributes: MemAttributes::Device,
            access_permissions: Access::ReadWrite,
            execute: ExecutionPolicy::Never
        }
    }
]);

fn rx_range_inclusive() -> RangeInclusive<usize> {
    RangeInclusive::new(rx_start(), rx_end_exclusive() - 1)
}

fn remapped_mmio_range_inclusive() -> RangeInclusive<usize> {
    RangeInclusive::new(0x1FFF_0000, 0x1FFF_FFFF)
}

fn mmio_range_inclusive() -> RangeInclusive<usize> {
    RangeInclusive::new(MMIO_BASE, MMIO_ENDI)
}
