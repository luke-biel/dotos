use crate::arch::aarch64::barrier::isb;
use core::time::Duration;

pub struct Timer;

impl Timer {
    pub fn time_since_start(&self) -> Duration {
        let time = unsafe { hardware_time() } * 1_000_000_000;
        let freq = unsafe { frequency() } as u64;

        Duration::from_nanos(time / freq)
    }
}

unsafe fn hardware_time() -> u64 {
    isb(|| {
        let value: u64;
        asm!("mrs {}, CNTPCT_EL0", out(reg) value, options(nomem, nostack));
        value
    })
}

unsafe fn frequency() -> u32 {
    let value: u32;
    asm!("mrs {0:x}, CNTFRQ_EL0", out(reg) value, options(nomem, nostack));
    value
}
