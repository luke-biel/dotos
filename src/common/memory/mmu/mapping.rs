use crate::{
    common::{
        memory::{
            mmu::descriptors::{Attributes, MMIODescriptor, MemoryAttributes, PageSliceDescriptor},
            Address,
            Physical,
            Virtual,
        },
        sync::{InitStateLock, ReadWriteLock},
    },
    info,
    print,
    println,
};

#[derive(Clone, Debug)]
pub struct MappingRecordEntry {
    pub users: [Option<&'static str>; 5],
    pub pages: PageSliceDescriptor<Physical>,
    pub start_addr: Address<Virtual>,
    pub attributes: Attributes,
}

pub struct MappingRecord {
    items: [Option<MappingRecordEntry>; 12],
}

pub static KERNEL_MAPPING_RECORD: InitStateLock<MappingRecord> =
    InitStateLock::new(MappingRecord::new());

impl MappingRecordEntry {
    pub fn new(
        name: &'static str,
        vpages: PageSliceDescriptor<Virtual>,
        ppages: PageSliceDescriptor<Physical>,
        attributes: Attributes,
    ) -> Self {
        Self {
            users: [Some(name), None, None, None, None],
            pages: ppages,
            start_addr: vpages.start_addr(),
            attributes,
        }
    }

    fn next_free_user_mut(&mut self) -> Result<&mut Option<&'static str>, &'static str> {
        if let Some(item) = self.users.iter_mut().find(|x| x.is_none()) {
            Ok(item)
        } else {
            Err("No more space for user info storage")
        }
    }

    pub fn add_user(&mut self, user: &'static str) -> Result<(), &'static str> {
        let user_slot = self.next_free_user_mut()?;
        *user_slot = Some(user);
        Ok(())
    }
}

impl MappingRecord {
    pub const fn new() -> Self {
        const DEFAULT: Option<MappingRecordEntry> = None;
        Self {
            items: [DEFAULT; 12],
        }
    }

    fn next_free_entry_mut(&mut self) -> Result<&mut Option<MappingRecordEntry>, &'static str> {
        if let Some(item) = self.items.iter_mut().find(|i| i.is_none()) {
            Ok(item)
        } else {
            Err("No more space for mapping info storage")
        }
    }

    fn find_duplicate_mut(
        &mut self,
        pages: PageSliceDescriptor<Physical>,
    ) -> Option<&mut MappingRecordEntry> {
        self.items
            .iter_mut()
            .flatten()
            .filter(|i| i.attributes.memory == MemoryAttributes::Device)
            .find(|i| i.pages == pages)
    }

    pub fn add(
        &mut self,
        name: &'static str,
        vpages: PageSliceDescriptor<Virtual>,
        ppages: PageSliceDescriptor<Physical>,
        attr: Attributes,
    ) -> Result<(), &'static str> {
        let next = self.next_free_entry_mut()?;
        *next = Some(MappingRecordEntry::new(name, vpages, ppages, attr));
        Ok(())
    }

    pub fn print_status(&self) {
        info!("memory mapping:");
        for entry in self.items.iter().flatten() {
            info!(
                "  - physical: {}..{}\n                    \
              virtual: {}..{}\n                    \
              attributes: {}\n                    \
              users:",
                entry.pages.start_addr(),
                entry.pages.endi_addr(),
                entry.start_addr,
                entry.start_addr + (entry.pages.size() - 1),
                entry.attributes
            );
            let mut nl = false;
            for user in entry.users.iter().flatten() {
                print!(
                    "{}                      - `{}`",
                    if nl { "\n" } else { "" },
                    user
                );
                nl = true;
            }
            println!();
        }
    }
}

pub fn kernel_add(
    name: &'static str,
    vpages: PageSliceDescriptor<Virtual>,
    ppages: PageSliceDescriptor<Physical>,
    attr: Attributes,
) -> Result<(), &'static str> {
    KERNEL_MAPPING_RECORD.map_write(|i| i.add(name, vpages, ppages, attr))
}

pub fn find_and_insert_mmio_duplicate(
    descriptor: MMIODescriptor,
    user: &'static str,
) -> Option<Address<Virtual>> {
    let page: PageSliceDescriptor<Physical> = descriptor.into();

    KERNEL_MAPPING_RECORD.map_write(|i| {
        let dup = i.find_duplicate_mut(page)?;

        if let Err(err) = dup.add_user(user) {
            crate::warn!("{}", err);
        }

        Some(dup.start_addr)
    })
}
