use crate::arch::cpu::wait_forever;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    wait_forever()
}
