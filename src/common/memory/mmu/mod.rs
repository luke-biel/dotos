use core::ops::RangeInclusive;

pub trait MemoryManagementUnit {
    unsafe fn enable_mmu_and_caching(&self) -> Result<(), &'static str>;
    fn is_enabled(&self) -> bool;
}

pub struct TranslationGranule<const SIZE: usize>;
pub struct AddressSpace<const SIZE: usize>;

#[derive(Copy, Clone, Debug)]
pub enum Translation {
    Id,
    Offset(usize),
}

#[derive(Copy, Clone, Debug)]
pub enum MemoryAttributes {
    CacheableDRAM,
    Device,
}

#[derive(Copy, Clone, Debug)]
pub enum AccessPermissions {
    RX,
    RW,
}

#[derive(Copy, Clone, Debug)]
pub enum Execute {
    Always,
    Never,
}

#[derive(Copy, Clone, Debug)]
pub struct Attributes {
    pub memory: MemoryAttributes,
    pub access: AccessPermissions,
    pub execute: Execute,
}

pub struct TranslationDescriptor {
    pub name: &'static str,
    pub vrange: fn() -> RangeInclusive<usize>,
    pub prange_translation: Translation,
    pub attributes: Attributes,
}

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

impl Default for Attributes {
    fn default() -> Self {
        Self {
            memory: MemoryAttributes::CacheableDRAM,
            access: AccessPermissions::RW,
            execute: Execute::Never,
        }
    }
}
