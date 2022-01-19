use core::arch::asm;

use registers::mpidr_el1::core_id_el1;

use crate::{
    arch::arch_impl::cpu::{
        instructions::{eret, wfe},
        registers::{
            cnthctl_el2::CnthctlEl2,
            cntvoff_el2::CntvoffEl2,
            current_el::{current_el, ExceptionLevel},
            elr_el2::ElrEl2,
            hcr_el2::HcrEl2,
            sp_el1::SpEl1,
            spsr_el2::SpsrEl2,
        },
    },
    bsp::device::{
        cpu::BOOT_CORE_ID,
        memory::{boot_core_stack_ende, bss},
    },
};

pub mod exception;
pub mod instructions;
pub mod registers;

#[no_mangle]
unsafe fn _start() -> ! {
    if current_el() != ExceptionLevel::EL2 {
        park()
    }

    if BOOT_CORE_ID != core_id_el1() {
        park()
    }

    for region in bss() {
        asm!("stp xzr, xzr, [{}], #16", in(reg) region, options(nostack));
    }

    prepare_kernel();
}

#[inline(always)]
unsafe fn prepare_kernel() -> ! {
    enter_el1()
}

/// Method prepares register values for el2 -> el1 change and then entries `init` function
#[inline(always)]
unsafe fn enter_el1() -> ! {
    CnthctlEl2::new().set(0b11);
    CntvoffEl2::new().set(0);
    HcrEl2::new().set(1 << 31); // Zero hcr_el2 register and set RW to EL1AArch64
    SpsrEl2::new().set(0b111100101);
    ElrEl2::new().set(crate::kernel_init as *const () as u64);
    SpEl1::new().set(boot_core_stack_ende().addr() as u64);

    eret()
}

#[no_mangle]
pub unsafe fn park() -> ! {
    loop {
        wfe()
    }
}
