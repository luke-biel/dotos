use core::{marker::PhantomData, ops::RangeInclusive};

use descriptors::{
    AccessPermissions,
    Attributes,
    Execute,
    MemoryAttributes,
    Translation,
    TranslationDescriptor,
};

use crate::{
    bsp::device::memory::mmu::KernelGranule,
    common::{
        memory::{mmu::translation_table::TranslationTable, Address, AddressType, Physical},
        sync::ReadWriteLock,
    },
    statics,
};

pub mod descriptors;
pub mod translation_table;

pub trait MemoryManagementUnit {
    unsafe fn enable_mmu_and_caching(&self) -> Result<(), &'static str>;
    fn is_enabled(&self) -> bool;
}

pub struct TranslationGranule<const SIZE: usize>;
pub struct AddressSpace<const SIZE: usize>;

pub struct KernelVirtualLayout<const SIZE: usize> {
    max_vaddr: usize,
    layout: [TranslationDescriptor; SIZE],
}

impl<const SIZE: usize> TranslationGranule<SIZE> {
    pub const SIZE: usize = Self::size_checked();

    pub const SHIFT: usize = Self::SIZE.trailing_zeros() as usize;

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
impl<const SIZE: usize> KernelVirtualLayout<SIZE> {
    pub const fn new(max_vaddr: usize, layout: [TranslationDescriptor; SIZE]) -> Self {
        Self { max_vaddr, layout }
    }

    pub fn vaddr_properties(&self, vaddr: usize) -> Result<(usize, Attributes), &'static str> {
        if vaddr > self.max_vaddr {
            return Err("address out of range");
        }

        for i in self.layout.iter() {
            if (i.vrange)().contains(&vaddr) {
                let output_addr = match i.prange_translation {
                    Translation::Id => vaddr,
                    Translation::Offset(a) => a + (vaddr - (i.vrange)().start()),
                };

                return Ok((output_addr, i.attributes));
            }
        }

        Ok((vaddr, Attributes::default()))
    }
}

pub fn map_kernel_binary() -> Result<Address<Physical>, &'static str> {
    let kernel_base_addr = statics::KERNEL_TABLES.map_write(|tables| {
        tables.init();
        tables.base_addr()
    });

    bsp::map_kernel_binary()?;

    Ok(kernel_base_addr)
}
