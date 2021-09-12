use descriptors::Attributes;

use crate::{
    bsp::rpi3::memory::mmu::KernelGranule,
    common::{
        memory::{
            mmu::{
                descriptors::{
                    AccessPermissions,
                    Execute,
                    MMIODescriptor,
                    MemoryAttributes,
                    PageSliceDescriptor,
                },
                mapping::{find_and_insert_mmio_duplicate, kernel_add},
                translation_table::TranslationTable,
            },
            Address,
            Physical,
            Virtual,
        },
        statics::KERNEL_TABLES,
        sync::ReadWriteLock,
    },
    statics,
};

pub mod descriptors;
pub mod mapping;
pub mod translation_table;

pub trait MemoryManagementUnit {
    unsafe fn enable_mmu_and_caching(
        &self,
        translation_table_base_addr: Address<Physical>,
    ) -> Result<(), &'static str>;
    fn is_enabled(&self) -> bool;
}

pub struct TranslationGranule<const SIZE: usize>;
pub struct AddressSpace<const SIZE: usize>;

impl<const SIZE: usize> TranslationGranule<SIZE> {
    pub const SIZE: usize = Self::size_checked();

    pub const SHIFT: usize = Self::SIZE.trailing_zeros() as usize;

    pub const MASK: usize = Self::SIZE - 1;

    const fn size_checked() -> usize {
        assert!(SIZE.is_power_of_two());

        SIZE
    }
}

impl<const SIZE: usize> AddressSpace<SIZE> {
    pub const SIZE: usize = Self::size_checked();

    pub const SHIFT: usize = Self::SIZE.trailing_zeros() as usize;

    const fn size_checked() -> usize {
        assert!(SIZE.is_power_of_two());

        Self::arch_address_space_size_sanity_checks();

        SIZE
    }
}

pub fn map_kernel_pages_unchecked(
    name: &'static str,
    vpages: PageSliceDescriptor<Virtual>,
    ppages: PageSliceDescriptor<Physical>,
    attr: Attributes,
) -> Result<(), &'static str> {
    unsafe {
        KERNEL_TABLES.map_write(|tables| tables.map_pages(vpages, ppages, attr))?;
    }

    kernel_add_record(name, vpages, ppages, attr);

    Ok(())
}

pub fn kernel_add_record(
    name: &'static str,
    vpages: PageSliceDescriptor<Virtual>,
    ppages: PageSliceDescriptor<Physical>,
    attr: Attributes,
) {
    if let Err(err) = kernel_add(name, vpages, ppages, attr) {
        crate::warn!("{}", err);
    }
}

pub fn map_kernel_pages_at(
    name: &'static str,
    vpages: PageSliceDescriptor<Virtual>,
    ppages: PageSliceDescriptor<Physical>,
    attr: Attributes,
) -> Result<(), &'static str> {
    if KERNEL_TABLES.map_read(|tables| tables.is_page_slice_mmio(vpages)) {
        return Err("Cannot manualy map into mmio region");
    }

    map_kernel_pages_unchecked(name, vpages, ppages, attr)
}

pub fn map_kernel_binary() -> Result<Address<Physical>, &'static str> {
    let kernel_base_addr = statics::KERNEL_TABLES.map_write(|tables| {
        tables.init();
        tables.base_addr()
    });

    crate::bsp::device::memory::mmu::map_kernel_binary()?;

    Ok(kernel_base_addr)
}

pub fn map_kernel_mmio(
    compat: &'static str,
    descriptor: MMIODescriptor,
) -> Result<Address<Virtual>, &'static str> {
    let ppages: PageSliceDescriptor<Physical> = descriptor.into();
    let offset = descriptor.start_addr().addr() & KernelGranule::MASK;

    let addr = if let Some(addr) = find_and_insert_mmio_duplicate(descriptor, compat) {
        addr
    } else {
        let vpages: PageSliceDescriptor<Virtual> =
            KERNEL_TABLES.map_write(|tables| tables.next_page_slice(ppages.num_pages()))?;

        map_kernel_pages_unchecked(
            compat,
            vpages,
            ppages,
            Attributes {
                memory: MemoryAttributes::Device,
                access: AccessPermissions::RW,
                execute: Execute::Never,
            },
        )?;

        vpages.start_addr()
    };

    Ok(addr + offset)
}
