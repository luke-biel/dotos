use descriptors::{Attributes};

use crate::{
    common::{
        memory::{
            mmu::{descriptors::PageSliceDescriptor, translation_table::TranslationTable},
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
    _name: &'static str,
    vpages: PageSliceDescriptor<Virtual>,
    ppages: PageSliceDescriptor<Physical>,
    attr: Attributes,
) -> Result<(), &'static str> {
    unsafe {
        KERNEL_TABLES.map_write(|tables| tables.map_pages(vpages, ppages, attr))?;
    }

    // TODO kernel_add and whole 'mapping_record' module

    Ok(())
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
