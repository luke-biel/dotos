use core::ops::RangeInclusive;

#[derive(Copy, Clone, Debug)]
pub enum Translation {
    Identity,
    Offset(usize),
}

#[derive(Copy, Clone, Debug)]
pub enum MemAttributes {
    CacheableDRAM,
    Device,
}

#[derive(Copy, Clone, Debug)]
pub enum Access {
    ReadWrite,
    ReadOnly,
}

#[derive(Copy, Clone, Debug)]
pub enum ExecutionPolicy {
    Never,
    Always,
}

#[derive(Copy, Clone, Debug)]
pub struct Attributes {
    pub mem_attributes: MemAttributes,
    pub access_permissions: Access,
    pub execute: ExecutionPolicy,
}

pub struct TranslationDescriptor {
    pub name: &'static str,
    pub virtual_range: fn() -> RangeInclusive<usize>,
    pub translation: Translation,
    pub attributes: Attributes,
}

pub struct KernelVirtualLayout<const COUNT: usize> {
    max_addr: usize,

    inner: [TranslationDescriptor; COUNT],
}

impl<const COUNT: usize> KernelVirtualLayout<COUNT> {
    pub const fn new(max_addr: usize, layout: [TranslationDescriptor; COUNT]) -> Self {
        Self {
            max_addr,
            inner: layout,
        }
    }

    pub fn properties(&self, vaddr: usize) -> Result<(usize, Attributes), &'static str> {
        if vaddr > self.max_addr {
            return Err("Address is out of range");
        }

        for descriptor in self.inner.iter() {
            let addr_range = (descriptor.virtual_range)();
            if addr_range.contains(&vaddr) {
                let paddr = match descriptor.translation {
                    Translation::Identity => vaddr,
                    Translation::Offset(offset) => offset + (vaddr + addr_range.start()),
                };

                return Ok((paddr, descriptor.attributes));
            }
        }

        Ok((
            vaddr,
            Attributes {
                mem_attributes: MemAttributes::CacheableDRAM,
                access_permissions: Access::ReadWrite,
                execute: ExecutionPolicy::Never,
            },
        ))
    }
}
