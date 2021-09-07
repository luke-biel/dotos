use crate::common::memory::{
    mmu::descriptors::{Attributes, PageSliceDescriptor},
    Address,
    Physical,
    Virtual,
};

pub trait TranslationTable {
    fn init(&mut self);
    fn base_addr(&self) -> Address<Physical>;
    unsafe fn map_pages(
        &mut self,
        vpages: PageSliceDescriptor<Virtual>,
        ppages: PageSliceDescriptor<Physical>,
        attributes: Attributes,
    ) -> Result<(), &'static str>;
    fn next_page_slice(
        &mut self,
        num_pages: usize,
    ) -> Result<PageSliceDescriptor<Virtual>, &'static str>;
    fn is_page_slice_mmio(&self, pages: PageSliceDescriptor<Virtual>) -> bool;
}
