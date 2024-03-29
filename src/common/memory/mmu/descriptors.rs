use core::{intrinsics::size_of, marker::PhantomData};

use derive_more::Display;

use crate::{
    bsp::device::memory::mmu::KernelGranule,
    common::memory::{Address, AddressType, Physical, Virtual},
};

#[derive(Copy, Clone, Debug, PartialEq, Display)]
pub enum MemoryAttributes {
    #[display(fmt = "C")]
    CacheableDRAM,
    #[display(fmt = "D")]
    Device,
}

#[derive(Copy, Clone, Debug, Display)]
#[allow(non_camel_case_types)]
pub enum AccessPermissions {
    RX,
    RW,
    RW_EL0,
}

#[derive(Copy, Clone, Debug, Display)]
pub enum Execute {
    #[display(fmt = "E")]
    Allow,
    #[display(fmt = "N")]
    Never,
}

#[derive(Copy, Clone, Debug, Display)]
#[display(fmt = "{}{}{}", memory, access, execute)]
pub struct Attributes {
    pub memory: MemoryAttributes,
    pub access: AccessPermissions,
    pub execute: Execute,
}

#[repr(C)]
pub struct Page<A: AddressType> {
    inner: [u8; KernelGranule::SIZE],
    _phantom: PhantomData<A>,
}

#[derive(Copy, Clone, Debug, PartialEq, Display)]
#[repr(C)]
#[display(fmt = "{}..{}", start, "self.endi_addr()")]
pub struct PageSliceDescriptor<A: AddressType> {
    start: Address<A>,
    num_pages: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct PageSliceDescriptorIter<A: AddressType> {
    ptr: *const Page<A>,
    remaining: usize,
    len: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct MMIODescriptor {
    addr: Address<Physical>,
    size: usize,
}

impl MMIODescriptor {
    pub const fn new(addr: Address<Physical>, size: usize) -> Self {
        assert!(size > 0);

        Self { addr, size }
    }

    pub const fn start_addr(&self) -> Address<Physical> {
        self.addr
    }

    pub const fn end_addr(&self) -> Address<Physical> {
        self.addr + (self.size - 1)
    }
}

impl From<MMIODescriptor> for PageSliceDescriptor<Physical> {
    fn from(desc: MMIODescriptor) -> Self {
        let start = desc.addr.align_down::<{ KernelGranule::SHIFT }>();
        let num_pages = ((desc.end_addr().addr() - start.addr()) >> KernelGranule::SHIFT) + 1;

        Self { start, num_pages }
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

impl<A: AddressType> PageSliceDescriptor<A> {
    pub const fn from_addr(address: Address<A>, num_pages: usize) -> Self {
        Self {
            start: address,
            num_pages,
        }
    }

    const fn first_page_ptr(&self) -> *const Page<A> {
        self.start.addr() as *const _
    }

    pub fn num_pages(&self) -> usize {
        self.num_pages
    }

    pub fn size(&self) -> usize {
        self.num_pages * KernelGranule::SIZE
    }

    pub fn start_addr(&self) -> Address<A> {
        self.start
    }

    pub fn endi_addr(&self) -> Address<A> {
        self.start + (self.size() - 1)
    }

    pub unsafe fn as_slice(&self) -> &[Page<A>] {
        core::slice::from_raw_parts(self.first_page_ptr(), self.num_pages)
    }
}

impl From<PageSliceDescriptor<Virtual>> for PageSliceDescriptor<Physical> {
    fn from(desc: PageSliceDescriptor<Virtual>) -> Self {
        Self {
            start: Address::new(desc.start.addr()),
            num_pages: desc.num_pages,
        }
    }
}

impl<A: AddressType> Iterator for PageSliceDescriptorIter<A> {
    type Item = Page<A>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            let val = unsafe { self.ptr.read() };
            self.ptr = self.ptr.wrapping_add(size_of::<Self::Item>());
            self.remaining -= 1;
            Some(val)
        }
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if self.len == 0 {
            None
        } else {
            unsafe {
                Some(
                    self.ptr
                        .wrapping_add(size_of::<Self::Item>() * self.remaining)
                        .read(),
                )
            }
        }
    }
}

impl<A: AddressType> ExactSizeIterator for PageSliceDescriptorIter<A> {
    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<A: AddressType> Page<A> {
    pub fn addr(&self) -> usize {
        self.inner.as_ptr() as usize
    }
}
