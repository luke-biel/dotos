use crate::common::mem::zero_region_volatile;
use crate::bsp::mem::bss_section;
use crate::kernel_init;

unsafe fn clear_bss() {
    zero_region_volatile(bss_section());
}

pub unsafe fn runtime_init() -> ! {
    clear_bss();

    kernel_init()
}
