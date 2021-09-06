use crate::{
    arch::aarch64::cpu::{
        instructions::{eret, wfe},
        registers::{core_id_el1, current_el, ExceptionLevel},
    },
    bsp::rpi3::{
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
    let cnthctl_el2 = 0b11_u64;
    asm!("msr cnthctl_el2, {}", in(reg) cnthctl_el2, options(nostack, nomem));

    let cntvoff_el2 = 0_u64;
    asm!("msr cntvoff_el2, {}", in(reg) cntvoff_el2, options(nostack, nomem));

    let hcr_el2 = 1_u64 << 31;
    asm!("msr hcr_el2, {}", in(reg) hcr_el2, options(nostack, nomem));

    let spsr_el2 = 0b111100101_u64;
    asm!("msr spsr_el2, {}", in(reg) spsr_el2, options(nostack, nomem));

    let init = crate::kernel_init as *const () as u64;
    asm!("msr elr_el2, {}", in(reg) init, options(nostack, nomem));

    let boot_core_stack_ende = boot_core_stack_ende();
    asm!("msr sp_el1, {}", in(reg) boot_core_stack_ende, options(nostack, nomem));

    eret()
}

#[no_mangle]
pub unsafe fn park() -> ! {
    loop {
        wfe()
    }
}
