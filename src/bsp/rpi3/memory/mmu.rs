use crate::{
    bsp::device::memory::{
        boot_core_stack_size,
        boot_core_stack_start,
        rw_size,
        rw_start,
        rx_size,
        rx_start,
    },
    common::{
        memory::{
            mmu::{
                descriptors::{
                    AccessPermissions,
                    Attributes,
                    Execute,
                    MemoryAttributes,
                    PageSliceDescriptor,
                },
                map_kernel_pages_at,
                AddressSpace,
                TranslationGranule,
            },
            Physical,
            Virtual,
        },
        statics::KERNEL_TABLES,
        sync::ReadWriteLock,
    },
};

pub type KernelGranule = TranslationGranule<{ 64 * 1024 }>;
pub type KernelAddrSpace = AddressSpace<{ 8 * 1024 * 1024 * 1024 }>;

const fn size_to_num_pages(size: usize) -> usize {
    assert!(size > 0);
    assert!(size % KernelGranule::SIZE == 0);

    size >> KernelGranule::SHIFT
}

fn rx_vpage_desc() -> PageSliceDescriptor<Virtual> {
    PageSliceDescriptor::from_addr(rx_start(), size_to_num_pages(rx_size()))
}

fn rx_ppage_desc() -> PageSliceDescriptor<Physical> {
    rx_vpage_desc().into()
}

fn rw_vpage_desc() -> PageSliceDescriptor<Virtual> {
    PageSliceDescriptor::from_addr(rw_start(), size_to_num_pages(rw_size()))
}

fn rw_ppage_desc() -> PageSliceDescriptor<Physical> {
    rw_vpage_desc().into()
}

fn boot_core_stack_vpage_desc() -> PageSliceDescriptor<Virtual> {
    PageSliceDescriptor::from_addr(
        boot_core_stack_start(),
        size_to_num_pages(boot_core_stack_size()),
    )
}

fn boot_core_stack_ppage_desc() -> PageSliceDescriptor<Physical> {
    boot_core_stack_vpage_desc().into()
}

pub fn map_kernel_binary() -> Result<(), &'static str> {
    map_kernel_pages_at(
        "kernel code + RO data",
        rx_vpage_desc(),
        rx_ppage_desc(),
        Attributes {
            memory: MemoryAttributes::CacheableDRAM,
            access: AccessPermissions::RX,
            execute: Execute::Allow,
        },
    )
    .expect("map Kernel Code & RO data");

    map_kernel_pages_at(
        "kernel data + BSS",
        rw_vpage_desc(),
        rw_ppage_desc(),
        Attributes {
            memory: MemoryAttributes::CacheableDRAM,
            access: AccessPermissions::RW,
            execute: Execute::Never,
        },
    )
    .expect("map Kernel Data & BSS");

    map_kernel_pages_at(
        "kernel boot-core stack",
        boot_core_stack_vpage_desc(),
        boot_core_stack_ppage_desc(),
        Attributes {
            memory: MemoryAttributes::CacheableDRAM,
            access: AccessPermissions::RW,
            execute: Execute::Never,
        },
    )
    .expect("map Kernel BOOT-CORE stack");

    Ok(())
}
