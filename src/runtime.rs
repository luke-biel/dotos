use crate::common::mem::zero_region_volatile;
use crate::kernel_init;
use crate::bsp::raspberry_pi_3::mem::bss_section;

unsafe fn clear_bss() {
    zero_region_volatile(bss_section());
}

pub unsafe fn runtime_init() -> ! {
    clear_bss();

    kernel_init()
}
