use crate::common::memory::mmu::{Access, Attributes, ExecutionPolicy, MemAttributes};
use tock_registers::fields::FieldValue;
use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::register_bitfields;
use tock_registers::registers::InMemoryRegister;
use crate::arch::aarch64::memory::mmu::{Granule64KB, mair, Granule512MB};
use crate::bsp::raspberry_pi_3::memory::mmu::{LAYOUT, KernelAddressSpace};

register_bitfields! {u64,
    STAGE1_TABLE_DESCRIPTOR [
        NEXT_LEVEL_TABLE_ADDR_64KiB OFFSET(16) NUMBITS(32) [], // [47:16]

        TYPE OFFSET(1) NUMBITS(1) [
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
        UXN OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        PXN OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        OUTPUT_ADDR_64KiB OFFSET(16) NUMBITS(32) [], // [47:16]

        AF OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        SH OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],

        AP OFFSET(6) NUMBITS(2) [
            RW_EL1 = 0b00,
            RW_EL1_EL0 = 0b01,
            RO_EL1 = 0b10,
            RO_EL1_EL0 = 0b11
        ],

        AttrIndx OFFSET(2) NUMBITS(3) [],

        TYPE OFFSET(1) NUMBITS(1) [
            Reserved_Invalid = 0,
            Page = 1
        ],

        VALID OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TableDescriptor(u64);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PageDescriptor(u64);

pub const NUM_LVL2_TABLES: usize = KernelAddressSpace::SIZE >> Granule512MB::SHIFT;,

#[repr(C)]
#[repr(align(65536))]
pub struct FixedSizeTranslationTable<const NUM: usize> {
    level_3: [[PageDescriptor; 8192]; NUM],
    level_2: [TableDescriptor; NUM],
}

pub type KernelTranslationTable = FixedSizeTranslationTable<NUM_LVL2_TABLES>;

pub trait StartAddr {
    fn phys_start_addr_u64(&self) -> u64;
    fn phys_start_addr_usize(&self) -> usize;
}

impl<T, const N: usize> StartAddr for [T; N] {
    fn phys_start_addr_u64(&self) -> u64 {
        self as *const _ as u64
    }

    fn phys_start_addr_usize(&self) -> usize {
        self as *const _ as usize
    }
}

impl TableDescriptor {
    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn from_next_level_table_addr(paddr: usize) -> Self {
        let val = InMemoryRegister::<u64, STAGE1_TABLE_DESCRIPTOR::Register>::new(0);

        let shifted = paddr as u64 >> Granule64KB::SHIFT;
        val.write(
            STAGE1_TABLE_DESCRIPTOR::NEXT_LEVEL_TABLE_ADDR_64KiB.val(shifted)
                + STAGE1_TABLE_DESCRIPTOR::TYPE::Table
                + STAGE1_TABLE_DESCRIPTOR::VALID::True,
        );

        Self(val.get())
    }
}

impl From<Attributes> for FieldValue<u64, STAGE1_PAGE_DESCRIPTOR::Register> {
    fn from(attributes: Attributes) -> Self {
        let mut desc = match attributes.mem_attributes {
            MemAttributes::CacheableDRAM => {
                STAGE1_PAGE_DESCRIPTOR::SH::InnerShareable
                    + STAGE1_PAGE_DESCRIPTOR::AttrIndx.val(mair::NORMAL)
            }
            MemAttributes::Device => {
                STAGE1_PAGE_DESCRIPTOR::SH::OuterShareable
                    + STAGE1_PAGE_DESCRIPTOR::AttrIndx.val(mair::DEVICE)
            }
        };

        desc += match attributes.access_permissions {
            Access::ReadWrite => STAGE1_PAGE_DESCRIPTOR::AP::RW_EL1,
            Access::ReadOnly => STAGE1_PAGE_DESCRIPTOR::AP::RO_EL1,
        };

        desc += match attributes.execute {
            ExecutionPolicy::Never => STAGE1_PAGE_DESCRIPTOR::PXN::True,
            ExecutionPolicy::Always => STAGE1_PAGE_DESCRIPTOR::PXN::False,
        };

        desc += STAGE1_PAGE_DESCRIPTOR::UXN::True;

        desc
    }
}

impl PageDescriptor {
    pub const fn zero() -> Self {
        Self(0)
    }

    pub fn from_output_addr(paddr: usize, attributes: Attributes) -> Self {
        let val = InMemoryRegister::<u64, STAGE1_PAGE_DESCRIPTOR::Register>::new(0);

        let shifted = paddr as u64 >> Granule64KB::SHIFT;
        val.write(
            STAGE1_PAGE_DESCRIPTOR::OUTPUT_ADDR_64KiB.val(shifted)
                + STAGE1_PAGE_DESCRIPTOR::AF::True
                + STAGE1_PAGE_DESCRIPTOR::TYPE::Page
                + STAGE1_PAGE_DESCRIPTOR::VALID::True
                + attributes.into(),
        );

        Self(val.get())
    }
}

impl<const NUM: usize> FixedSizeTranslationTable<NUM> {
    pub const fn new() -> Self {
        assert!(NUM > 0);

        Self {
            level_2: [TableDescriptor::zero(); NUM],
            level_3: [[PageDescriptor::zero(); 8192]; NUM],
        }
    }

    pub unsafe fn populate_table_entries(&mut self) -> Result<(), &'static str> {
        for (l2_idx, l2_entry) in self.level_2.iter_mut().enumerate() {
            *l2_entry = TableDescriptor::from_next_level_table_addr(
                self.level_3[l2_idx].phys_start_addr_usize(),
            );

            for (l3_idx, l3_entry) in self.level_3[l2_idx].iter_mut().enumerate() {
                let vaddr = (l2_idx << Granule512MB::SHIFT) + (l3_idx << Granule64KB::SHIFT);

                let (paddr, attributes) = LAYOUT.properties(vaddr)?;

                *l3_entry = PageDescriptor::from_output_addr(paddr, attributes);
            }
        }

        Ok(())
    }

    pub fn base_paddr(&self) -> u64 {
        self.level_2.phys_start_addr_u64()
    }
}
