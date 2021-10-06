use crate::{
    arch::{
        aarch64::cpu::{
            instructions::{eret, wfe},
            registers::{
                cnthctl_el2::CnthctlEl2,
                cntvoff_el2::CntvoffEl2,
                core_id_el1,
                current_el,
                hcr_el2::HcrEl2,
            },
        },
        arch_impl::cpu::registers::current_el::ExceptionLevel,
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
    // TODO: Create facade around these registers
    CnthctlEl2::new().set(0b11);
    CntvoffEl2::new().set(0);
    HcrEl2::new().set(1 << 31); // Zero hcr_el2 register and set RW to EL1AArch64

    let spsr_el2 = 0b111100101_u64;
    asm!("msr spsr_el2, {}", in(reg) spsr_el2, options(nostack, nomem));

    let init = crate::kernel_init as *const () as u64;
    asm!("msr elr_el2, {}", in(reg) init, options(nostack, nomem));

    let boot_core_stack_ende = boot_core_stack_ende().addr();
    asm!("msr sp_el1, {}", in(reg) boot_core_stack_ende, options(nostack, nomem));

    eret()
}

#[no_mangle]
pub unsafe fn park() -> ! {
    loop {
        wfe()
    }
}
