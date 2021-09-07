use core::convert;

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
    registers::InMemoryRegister,
};

use crate::{
    arch::arch_impl::memory::mmu::{mair, Granule512MB, Granule64KB},
    bsp::{
        device::memory::{map::END, mmu::KernelAddrSpace},
        rpi3::statics::KERNEL_VIRTUAL_LAYOUT,
    },
    common::memory::{
        mmu::{
            descriptors::{
                AccessPermissions,
                Attributes,
                Execute,
                MemoryAttributes,
                Page,
                PageSliceDescriptor,
            },
            translation_table::TranslationTable,
        },
        Address,
        Physical,
        Virtual,
    },
};

register_bitfields! {u64,
    STAGE1_TABLE_DESCRIPTOR [
        NEXT_LEVEL_TABLE_ADDR_64KB OFFSET(16) NUMBITS(32) [], // [47:16]

        TYPE  OFFSET(1) NUMBITS(1) [
            Block = 0,
            Table = 1
        ],

        VALID OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

register_bitfields! {u64,
    STAGE1_PAGE_DESCRIPTOR [
        UXN      OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        PXN      OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        OUTPUT_ADDR_64KB OFFSET(16) NUMBITS(32) [], // [47:16]

        AF       OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        SH       OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],

        AP       OFFSET(6) NUMBITS(2) [
            RW_EL1 = 0b00,
            RW_EL1_EL0 = 0b01,
            RO_EL1 = 0b10,
            RO_EL1_EL0 = 0b11
        ],

        AttrIndx OFFSET(2) NUMBITS(3) [],

        TYPE     OFFSET(1) NUMBITS(1) [
            Reserved_Invalid = 0,
            Page = 1
        ],

        VALID    OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

#[derive(Copy, Clone)]
#[repr(C)]
struct TableDescriptor {
    value: u64,
}

#[derive(Copy, Clone)]
#[repr(C)]
struct PageDescriptor {
    value: u64,
}

trait StartAddr {
    fn start_addr(&self) -> Address<Physical>;
}

const NUM_LVL2_TABLES: usize = KernelAddrSpace::SIZE >> Granule512MB::SHIFT;

#[repr(C)]
#[repr(align(65536))]
pub struct FixedSizeTranslationTable<const NUM_TABLES: usize> {
    lvl3: [[PageDescriptor; 8192]; NUM_TABLES],
    lvl2: [TableDescriptor; NUM_TABLES],
    current_l3_mmio_index: usize,
    is_initialized: bool,
}

pub type KernelTranslationTable = FixedSizeTranslationTable<NUM_LVL2_TABLES>;

impl<T, const N: usize> StartAddr for [T; N] {
    fn start_addr(&self) -> Address<Physical> {
        Address::new(self as *const _ as usize)
    }
}

impl TableDescriptor {
    pub const fn new_zeroed() -> Self {
        Self { value: 0 }
    }

    pub fn from_next_lvl_table_addr(next_lvl_table: Address<Physical>) -> Self {
        let val = InMemoryRegister::<u64, STAGE1_TABLE_DESCRIPTOR::Register>::new(0);

        let shifted = usize::from(next_lvl_table) >> Granule64KB::SHIFT;
        val.write(
            STAGE1_TABLE_DESCRIPTOR::NEXT_LEVEL_TABLE_ADDR_64KB.val(shifted as u64)
                + STAGE1_TABLE_DESCRIPTOR::TYPE::Table
                + STAGE1_TABLE_DESCRIPTOR::VALID::True,
        );

        TableDescriptor { value: val.get() }
    }
}

impl convert::From<Attributes>
    for tock_registers::fields::FieldValue<u64, STAGE1_PAGE_DESCRIPTOR::Register>
{
    fn from(attribute_fields: Attributes) -> Self {
        let mut desc = match attribute_fields.memory {
            MemoryAttributes::CacheableDRAM => {
                STAGE1_PAGE_DESCRIPTOR::SH::InnerShareable
                    + STAGE1_PAGE_DESCRIPTOR::AttrIndx.val(mair::NORMAL)
            }
            MemoryAttributes::Device => {
                STAGE1_PAGE_DESCRIPTOR::SH::OuterShareable
                    + STAGE1_PAGE_DESCRIPTOR::AttrIndx.val(mair::DEVICE)
            }
        };

        desc += match attribute_fields.access {
            AccessPermissions::RX => STAGE1_PAGE_DESCRIPTOR::AP::RO_EL1,
            AccessPermissions::RW => STAGE1_PAGE_DESCRIPTOR::AP::RW_EL1,
        };

        desc += match attribute_fields.execute {
            Execute::Always => STAGE1_PAGE_DESCRIPTOR::PXN::False,
            Execute::Never => STAGE1_PAGE_DESCRIPTOR::PXN::True,
        };

        desc += STAGE1_PAGE_DESCRIPTOR::UXN::True;

        desc
    }
}

impl PageDescriptor {
    pub const fn new_zeroed() -> Self {
        Self { value: 0 }
    }

    pub fn from_output_addr(phys_output_addr: usize, attribute_fields: Attributes) -> Self {
        let val = InMemoryRegister::<u64, STAGE1_PAGE_DESCRIPTOR::Register>::new(0);

        let shifted = phys_output_addr as u64 >> Granule64KB::SHIFT;
        val.write(
            STAGE1_PAGE_DESCRIPTOR::OUTPUT_ADDR_64KB.val(shifted)
                + STAGE1_PAGE_DESCRIPTOR::AF::True
                + STAGE1_PAGE_DESCRIPTOR::TYPE::Page
                + STAGE1_PAGE_DESCRIPTOR::VALID::True
                + attribute_fields.into(),
        );

        Self { value: val.get() }
    }

    pub fn is_valid(&self) -> bool {
        InMemoryRegister::<u64, STAGE1_PAGE_DESCRIPTOR::Register>::new(self.value)
            .is_set(STAGE1_PAGE_DESCRIPTOR::VALID)
    }
}

impl<const NUM_TABLES: usize> FixedSizeTranslationTable<NUM_TABLES> {
    const L2_MMIO_START_INDEX: usize = NUM_TABLES - 1;
    const L3_MMIO_START_INDEX: usize = 8192 / 2;

    pub const fn new() -> Self {
        assert!(NUM_TABLES > 0);

        Self {
            lvl3: [[PageDescriptor::new_zeroed(); 8192]; NUM_TABLES],
            lvl2: [TableDescriptor::new_zeroed(); NUM_TABLES],
            current_l3_mmio_index: 0,
            is_initialized: false,
        }
    }

    pub unsafe fn populate(&mut self) -> Result<(), &'static str> {
        for (l2_nr, l2_entry) in self.lvl2.iter_mut().enumerate() {
            *l2_entry =
                TableDescriptor::from_next_lvl_table_addr(self.lvl3[l2_nr].phys_start_addr_usize());

            for (l3_nr, l3_entry) in self.lvl3[l2_nr].iter_mut().enumerate() {
                let virt_addr = (l2_nr << Granule512MB::SHIFT) + (l3_nr << Granule64KB::SHIFT);

                let (phys_output_addr, attribute_fields) =
                    KERNEL_VIRTUAL_LAYOUT.vaddr_properties(virt_addr)?;

                *l3_entry = PageDescriptor::from_output_addr(phys_output_addr, attribute_fields);
            }
        }

        Ok(())
    }

    pub fn base_paddr(&self) -> u64 {
        self.lvl2.phys_start_addr_u64()
    }

    fn lvl2_lvl3_index_from(&self, page: &Page<Virtual>) -> Result<(usize, usize), &'static str> {
        let addr = page.addr();
        let lvl2i = addr >> Granule512MB::SHIFT;
        let lvl3i = (addr & Granule512MB::MASK) >> Granule64KB::SHIFT;

        if lvl2i >= NUM_TABLES {
            return Err("Virtual page out of bounds of translation table");
        }

        Ok((lvl2i, lvl3i))
    }

    fn page_descriptor(
        &mut self,
        addr: &Page<Virtual>,
    ) -> Result<&mut PageDescriptor, &'static str> {
        let (lvl2i, lvl3i) = self.lvl2_lvl3_index_from(addr)?;

        Ok(&mut self.lvl3[lvl2i][lvl3i])
    }

    fn mmio_start_addr(&self) -> Address<Virtual> {
        Address::new(
            (Self::L2_MMIO_START_INDEX << Granule512MB::SHIFT)
                | (Self::L3_MMIO_START_INDEX << Granule64KB::SHIFT),
        )
    }

    fn mmio_endi_addr(&self) -> Address<Virtual> {
        Address::new(
            (Self::L2_MMIO_START_INDEX << Granule512MB::SHIFT)
                | (8191 << Granule64KB::SHIFT)
                | (Granule64KB::SIZE - 1),
        )
    }
}

impl<const NUM_TABLES: usize> TranslationTable for FixedSizeTranslationTable<T> {
    fn init(&mut self) {
        if self.is_initialized {
            panic!("Translation tables are already initialized");
        }

        for (idx, entry) in self.lvl2.iter_mut().enumerate() {
            *entry =
                TableDescriptor::from_next_lvl_table_addr(self.lvl3[idx].phys_start_addr_usize());
        }

        self.current_l3_mmio_index = Self::L3_MMIO_START_INDEX;
        self.is_initialized = true;
    }

    fn base_addr(&self) -> Address<Physical> {
        self.lvl2.start_addr()
    }

    unsafe fn map_pages(
        &mut self,
        vpages: PageSliceDescriptor<Virtual>,
        ppages: PageSliceDescriptor<Physical>,
        attributes: Attributes,
    ) -> Result<(), &'static str> {
        if !self.is_initialized {
            return Err("Translation table is uninitialized");
        }

        let v = vpages.iter();
        let p = ppages.iter();

        if v.len() != p.len() {
            return Err("Mismatched lengths of virtual and physical page slices");
        }

        if v.is_empty() {
            return Ok(());
        }

        if p.last().unwrap().addr() >= END.addr() {
            return Err("Tried to map outside address space");
        }

        for (ppage, vpage) in p.zip(v) {
            let descriptor = self.page_descriptor(&vpage)?;
            if descriptor.is_valid() {
                return Err("Virtual page already mapped");
            }

            *descriptor = PageDescriptor::from_output_addr(ppage.addr(), attributes);
        }

        Ok(())
    }

    fn next_page_slice(
        &mut self,
        num_pages: usize,
    ) -> Result<PageSliceDescriptor<Virtual>, &'static str> {
        if !self.is_initialized {
            return Err("Translation table is uninitialized");
        }

        if num_pages == 0 {
            return Err("num_pages = 0");
        }

        // TODO: Put this magic number somewhere
        if (self.current_l3_mmio_index + num_pages) > 8192 {
            return Err("No more MMIO space");
        }

        let addr = Address::new(
            (Self::L2_MMIO_START_INDEX << Granule512MB::SHIFT)
                | (self.current_l3_mmio_index << Granule64KB::SHIFT),
        );
        self.current_l3_mmio_index += num_pages;

        Ok(PageSliceDescriptor::from_addr(addr, num_pages))
    }

    fn is_page_slice_mmio(&self, pages: PageSliceDescriptor<Virtual>) -> bool {
        let mmio_range = (self.mmio_start_addr()..self.mmio_end_address());
        mmio_range.contains(&pages.start_addr()) && mmio_range.contains(&pages.end_addr())
    }
}
