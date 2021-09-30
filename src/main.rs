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
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(const_for)]
#![feature(const_mut_refs)]
#![feature(const_maybe_uninit_write)]
#![feature(once_cell)]

use core::intrinsics::volatile_load;
use core::time::Duration;

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
        scheduler::{spawn_process, SCHEDULER},
        state::KernelState,
        statics,
        sync::ReadWriteLock,
        time::scheduling::SchedulingManager,
    },
};
use crate::common::statics::CLOCK_TIMER;
use crate::common::time::clock::ClockManager;

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

    statics::BSP_DRIVER_MANAGER
        .init_early_drivers()
        .expect("early driver init");
    statics::BSP_DRIVER_MANAGER
        .post_early_drivers()
        .expect("post early driver init");
    statics::BSP_DRIVER_MANAGER
        .init_late_drivers()
        .expect("late driver init");
    statics::BSP_DRIVER_MANAGER
        .register_irq_handlers()
        .expect("driver register_irq_handler");

    statics::SYSTEM_TIMER_DRIVER
        .register_handler(&SCHEDULER)
        .expect("register ticks for scheduler");

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
    statics::KERNEL_MAPPING_RECORD.map_read(|r| r.print_status());

    // // TEMP
    // let big_addr: u64 = 1024 * 1024 * 1024 * 8;
    // unsafe {
    //     core::ptr::read_volatile(big_addr as *mut u64);
    // }
    // info!("Recovery from exception successful");
    // // TEMP end

    info!("current privilege level: {}", current_privilege_level());
    info!("exception status: {}", get_mask_state());

    spawn_process(|| {}).expect("spawn INIT process");

    spawn_process(test1).expect("spawn test process");
    spawn_process(test2).expect("spawn test process");
    loop {
        park()
    }
}

fn test1() {
    let mut x = 1;
    loop {
        crate::info!("process 1) {}", x);
        x += 1;
    }
}

fn test2() {
    let mut x = 1;
    loop {
        crate::info!("process 2) {}", x);
        x += 1;
    }
}
