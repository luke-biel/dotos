#![no_std]
#![no_main]
#![feature(asm)]
#![feature(crate_visibility_modifier)]
#![feature(core_intrinsics)]
#![feature(const_panic)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(const_trait_impl)]
#![feature(const_default_impls)]
#![feature(min_specialization)]
#![feature(const_fn_trait_bound)]
#![feature(exact_size_is_empty)]

use arch::aarch64::cpu::exception::current_privilege_level;

use crate::{
    arch::{
        aarch64::cpu::exception::asynchronous::local_irq_set_mask,
        arch_impl::cpu::{
            exception::{asynchronous::get_mask_state, init_exception_handling},
            park,
        },
    },
    common::{
        driver::DriverManager,
        memory::mmu::{map_kernel_binary, MemoryManagementUnit},
        state::KernelState,
        statics,
    },
};

crate mod arch;
mod bsp;
crate mod common;
mod log;
mod panic;

unsafe fn kernel_init() -> ! {
    init_exception_handling();
    let kernel_addr = map_kernel_binary().expect("map kernel binary");

    statics::MMU
        .enable_mmu_and_caching(kernel_addr)
        .expect("mmu init");

    statics::BSP_DRIVER_MANAGER.init().expect("driver init");
    statics::BSP_DRIVER_MANAGER
        .late_init()
        .expect("driver late_init");
    statics::BSP_DRIVER_MANAGER
        .register_irq_handlers()
        .expect("driver register_irq_handler");

    local_irq_set_mask(false);

    statics::STATE_MANAGER.transition(KernelState::Init, KernelState::SingleCoreRun);

    kernel_main()
}

unsafe fn kernel_main() -> ! {
    info!(
        "{} - v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    info!("build time: {}", env!("BUILD_DATE"));
    info!("git head: {}", env!("GIT_HASH"));

    statics::BSP_DRIVER_MANAGER.print_status();
    statics::INTERRUPT_CONTROLLER.print_status();

    // // TEMP
    // let big_addr: u64 = 1024 * 1024 * 1024 * 8;
    // unsafe {
    //     core::ptr::read_volatile(big_addr as *mut u64);
    // }
    // info!("Recovery from exception successful");
    // // TEMP end

    info!("current privilege level: {}", current_privilege_level());
    info!("exception status: {}", get_mask_state());

    park()
}
