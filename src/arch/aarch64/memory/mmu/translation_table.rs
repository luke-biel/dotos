use core::{convert, fmt::Formatter};

use tock_registers::{
    interfaces::{ReadWriteable, Readable, Writeable},
    register_bitfields,
    registers::InMemoryRegister,
};

use crate::{
    arch::arch_impl::memory::mmu::{mair, Granule512MB, Granule64KB},
    bsp::{
        device::memory::{
            map::{user::LOW_MEMORY, END},
            mmu::KernelAddrSpace,
        },
        rpi3::memory::{map::user::PAGE_COUNT, mmu::KernelGranule},
    },
    common::{
        memory::{
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
        statics::KERNEL_TABLES,
        sync::Mutex,
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
pub struct PageDescriptor {
    value: u64,
}

impl core::fmt::Debug for PageDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "0x{:x}", self.value)
    }
}

trait StartAddr {
    fn start_addr(&self) -> Address<Physical>;
}

const NUM_LVL2_TABLES: usize = KernelAddrSpace::SIZE >> Granule512MB::SHIFT;

#[repr(C)]
#[repr(align(65536))]
pub struct FixedSizeTranslationTable<const NUM_TABLES: usize> {
    // 64 KB windows per page entry
    pub lvl3: [[PageDescriptor; 8192]; NUM_TABLES],
    // 512 MB descriptors
    lvl2: [TableDescriptor; NUM_TABLES],
    current_l3_user_index: usize,
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

        let shifted = next_lvl_table.addr() >> Granule64KB::SHIFT;
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
            AccessPermissions::RW_EL0 => STAGE1_PAGE_DESCRIPTOR::AP::RW_EL1_EL0,
        };

        desc += match attribute_fields.execute {
            Execute::Allow => STAGE1_PAGE_DESCRIPTOR::PXN::False,
            Execute::Never => STAGE1_PAGE_DESCRIPTOR::PXN::True,
        };

        desc += match attribute_fields.execute {
            Execute::Allow => STAGE1_PAGE_DESCRIPTOR::UXN::False,
            Execute::Never => STAGE1_PAGE_DESCRIPTOR::UXN::True,
        };

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

    #[allow(clippy::assertions_on_constants)]
    pub const fn new() -> Self {
        assert!(KernelGranule::SIZE == Granule64KB::SIZE);
        assert!(NUM_TABLES > 0);

        Self {
            lvl3: [[PageDescriptor::new_zeroed(); 8192]; NUM_TABLES],
            lvl2: [TableDescriptor::new_zeroed(); NUM_TABLES],
            current_l3_mmio_index: 0,
            current_l3_user_index: 0,
            is_initialized: false,
        }
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

impl<const NUM_TABLES: usize> TranslationTable for FixedSizeTranslationTable<NUM_TABLES> {
    fn init(&mut self) {
        if self.is_initialized {
            panic!("Translation tables are already initialized");
        }

        for (idx, entry) in self.lvl2.iter_mut().enumerate() {
            *entry = TableDescriptor::from_next_lvl_table_addr(self.lvl3[idx].start_addr());
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
            return Err("map_pages: Translation table is uninitialized");
        }

        let v = vpages.as_slice();
        let p = ppages.as_slice();

        if v.len() != p.len() {
            return Err("map_pages: Mismatched lengths of virtual and physical page slices");
        }

        if v.is_empty() {
            return Ok(());
        }

        if p.last().expect("p last").addr() >= END.addr() {
            return Err("map_pages: Tried to map outside address space");
        }

        for (ppage, vpage) in p.iter().zip(v.iter()) {
            let descriptor = self.page_descriptor(vpage)?;
            if descriptor.is_valid() {
                crate::error!("{:x}, {:x}", ppage.addr(), vpage.addr());
                return Err("map_pages: Virtual page already mapped");
            }

            *descriptor = PageDescriptor::from_output_addr(ppage.addr(), attributes);
        }

        Ok(())
    }

    fn next_mmio_page_slice(
        &mut self,
        num_pages: usize,
    ) -> Result<PageSliceDescriptor<Virtual>, &'static str> {
        if !self.is_initialized {
            return Err("translation table is uninitialized");
        }

        if num_pages == 0 {
            return Err("num_pages = 0");
        }

        // TODO: Put this magic number somewhere
        if (self.current_l3_mmio_index + num_pages) > 8191 {
            return Err("no more MMIO space");
        }

        let addr = Address::new(
            (Self::L2_MMIO_START_INDEX << Granule512MB::SHIFT)
                | (self.current_l3_mmio_index << Granule64KB::SHIFT),
        );
        self.current_l3_mmio_index += num_pages;

        Ok(PageSliceDescriptor::from_addr(addr, num_pages))
    }

    fn next_user_page_slice(
        &mut self,
        num_pages: usize,
    ) -> Result<PageSliceDescriptor<Virtual>, &'static str> {
        if !self.is_initialized {
            return Err("next_page_slice: Translation table is uninitialized");
        }

        if num_pages == 0 {
            return Err("next_page_slice: num_pages = 0");
        }

        if (self.current_l3_user_index + num_pages) > PAGE_COUNT {
            return Err("out of memory");
        }

        let addr: usize = LOW_MEMORY.addr() + (self.current_l3_user_index * Granule64KB::SIZE);
        self.current_l3_user_index += num_pages;

        crate::trace!("registered {} user page(s) at {}", num_pages, addr);

        let ppages = PageSliceDescriptor::from_addr(Address::<Physical>::new(addr), num_pages);
        let vpages = PageSliceDescriptor::from_addr(Address::<Virtual>::new(addr), num_pages);
        let attributes = Attributes {
            memory: MemoryAttributes::CacheableDRAM,
            access: AccessPermissions::RW_EL0,
            execute: Execute::Never,
        };

        unsafe { KERNEL_TABLES.map_locked(|kt| kt.map_pages(vpages, ppages, attributes))? };

        Ok(vpages)
    }

    fn is_page_slice_mmio(&self, pages: PageSliceDescriptor<Virtual>) -> bool {
        let mmio_range = self.mmio_start_addr()..=self.mmio_endi_addr();
        mmio_range.contains(&pages.start_addr()) && mmio_range.contains(&pages.endi_addr())
    }
}
