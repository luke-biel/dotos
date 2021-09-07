use core::{
    cmp::Ordering,
    convert::Infallible,
    intrinsics::size_of,
    iter::{
        Chain,
        Cloned,
        Copied,
        Cycle,
        Enumerate,
        Filter,
        FilterMap,
        FlatMap,
        Flatten,
        FromIterator,
        Fuse,
        Inspect,
        Intersperse,
        IntersperseWith,
        Map,
        MapWhile,
        Peekable,
        Product,
        Rev,
        Scan,
        Skip,
        SkipWhile,
        StepBy,
        Sum,
        Take,
        TakeWhile,
        TrustedRandomAccessNoCoerce,
        Zip,
    },
    marker::PhantomData,
    ops::{RangeInclusive, Try},
};

use crate::{
    bsp::device::memory::mmu::KernelGranule,
    common::memory::{Address, AddressType},
};

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

pub struct TranslationDescriptor {
    pub name: &'static str,
    pub vrange: fn() -> RangeInclusive<usize>,
    pub prange_translation: Translation,
    pub attributes: Attributes,
}

#[derive(Copy, Clone, Debug)]
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

pub struct PageSliceDescriptor<A: AddressType> {
    start: Address<A>,
    num_pages: usize,
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
        usize::from(&self.start) as *const _
    }

    pub fn iter(&self) -> PageSliceDescriptorIter<A> {
        PageSliceDescriptorIter::new(self.first_page_ptr(), self.num_pages)
    }

    pub fn size(&self) -> usize {
        self.num_pages * KernelGranule::SIZE
    }

    pub fn start_addr(&self) -> Address<A> {
        self.start
    }

    pub fn end_addr(&self) -> Address<A> {
        self.start + self.size()
    }
}

pub struct PageSliceDescriptorIter<A: AddressType> {
    ptr: *const Page<A>,
    remaining: usize,
    len: usize,
}

impl<A: AddressType> PageSliceDescriptorIter<A> {
    pub fn new(start: *const Page<A>, len: usize) -> Self {
        Self {
            ptr: start,
            remaining: len,
            len,
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
            unsafe {
                self.ptr += size_of::<Self::Item>();
            }
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
