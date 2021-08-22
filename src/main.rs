#![no_std]
#![no_main]
#![feature(asm)]
#![feature(crate_visibility_modifier)]
#![feature(core_intrinsics)]
#![feature(const_panic)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]
#![feature(global_asm)]

use crate::{
    arch::arch_impl::{
        cpu::exception::{asynchronous::get_mask_state, init_exception_handling},
        exception::current_privilege_level,
    },
    common::{
        driver::DriverManager,
        memory::mmu::MemoryManagementUnit,
        serial_console::Read,
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

    statics::MMU.enable_mmu_and_caching().expect("mmu init");

    statics::BSP_DRIVER_MANAGER.init().expect("driver init");
    statics::BSP_DRIVER_MANAGER
        .late_init()
        .expect("driver late_init");

    kernel_main()
}

unsafe fn kernel_main() -> ! {
    info!(
        "> {} - v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    info!("> build time: {}", env!("BUILD_DATE"));
    info!("> git head: {}", env!("GIT_HASH"));

    info!("> drivers loaded:");
    for (i, driver) in statics::BSP_DRIVER_MANAGER.drivers.iter().enumerate() {
        info!("> {}: {}", i, driver.compat())
    }

    let privilege_level = current_privilege_level();
    info!("Current privilege level: {}", privilege_level);
    info!("Exception Status: {}", get_mask_state());

    unsafe {
        core::ptr::read_volatile((1_u64 << 33) as *mut u64);
    }

    let mut buf = [0u8; 512];
    let mut idx = 0;

    print!("> ");
    loop {
        let c = statics::UART_DRIVER.read_char() as u8;
        buf[idx] = c;
        idx += 1;
        if c == b'\n' {
            print!("\n");
            print!("(U) {}", core::str::from_utf8_unchecked(&buf[0..=idx]));
            print!("> ");
            idx = 0;
        } else {
            print!("{}", c as char);
        }
    }
}
