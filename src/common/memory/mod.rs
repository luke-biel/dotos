pub mod interface;
pub mod mmu;

pub struct TranslationGranule<const SIZE: usize>;
pub struct AddressSpace<const SIZE: usize>;

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
        Self::arch_address_space_size_sanity_check();

        SIZE
    }
}
