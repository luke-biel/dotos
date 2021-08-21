use core::convert;

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
    registers::InMemoryRegister,
};

use crate::{
    arch::arch_impl::memory::mmu::{mair, Granule512MB, Granule64KB},
    bsp::{device::memory::mmu::KernelAddrSpace, rpi3::statics::KERNEL_VIRTUAL_LAYOUT},
    common::memory::mmu::{AccessPermissions, Attributes, Execute, MemoryAttributes},
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
    fn phys_start_addr_u64(&self) -> u64;
    fn phys_start_addr_usize(&self) -> usize;
}

const NUM_LVL2_TABLES: usize = KernelAddrSpace::SIZE >> Granule512MB::SHIFT;

#[repr(C)]
#[repr(align(65536))]
pub struct FixedSizeTranslationTable<const NUM_TABLES: usize> {
    lvl3: [[PageDescriptor; 8192]; NUM_TABLES],
    lvl2: [TableDescriptor; NUM_TABLES],
}

pub type KernelTranslationTable = FixedSizeTranslationTable<NUM_LVL2_TABLES>;

impl<T, const N: usize> StartAddr for [T; N] {
    fn phys_start_addr_u64(&self) -> u64 {
        self as *const T as u64
    }

    fn phys_start_addr_usize(&self) -> usize {
        self as *const _ as usize
    }
}

impl TableDescriptor {
    pub const fn new_zeroed() -> Self {
        Self { value: 0 }
    }

    pub fn from_next_lvl_table_addr(phys_next_lvl_table_addr: usize) -> Self {
        let val = InMemoryRegister::<u64, STAGE1_TABLE_DESCRIPTOR::Register>::new(0);

        let shifted = phys_next_lvl_table_addr >> Granule64KB::SHIFT;
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
}

impl<const NUM_TABLES: usize> FixedSizeTranslationTable<NUM_TABLES> {
    pub const fn new() -> Self {
        assert!(NUM_TABLES > 0);

        Self {
            lvl3: [[PageDescriptor::new_zeroed(); 8192]; NUM_TABLES],
            lvl2: [TableDescriptor::new_zeroed(); NUM_TABLES],
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
}
