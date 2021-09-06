use core::time::Duration;

use crate::common::time_manager::TimeManager;

const NS_IN_S: u64 = 1_000_000_000;

pub struct GenericTimer;

impl GenericTimer {
    #[inline(always)]
    fn cntpct_el0(&self) -> u64 {
        let res: u64;
        unsafe {
            asm!("isb sy");
            asm!("mrs {}, cntpct_el0", out(reg) res, options(nostack, nomem));
        }
        res
    }

    #[inline(always)]
    fn cntfrq_el0(&self) -> u64 {
        let cntfrq_el0: u64;
        unsafe { asm!("mrs {}, cntfrq_el0", out(reg) cntfrq_el0, options(nostack, nomem)) };
        cntfrq_el0
    }
}

impl TimeManager for GenericTimer {
    fn resolution(&self) -> Duration {
        Duration::from_nanos(NS_IN_S / self.cntfrq_el0())
    }

    fn uptime(&self) -> Duration {
        Duration::from_nanos((self.cntpct_el0() * NS_IN_S) / self.cntfrq_el0())
    }

    fn sleep(&self, duration: Duration) {
        if duration.as_nanos() == 0 {
            return;
        }

        let frq = self.cntfrq_el0();
        let x = match frq.checked_mul(duration.as_nanos() as u64) {
            None => todo!("Warn here"),
            Some(val) => val,
        };
        let time = x / NS_IN_S;

        if time == 0 {
            todo!("Warn here 0")
        } else if time > u32::MAX as u64 {
            todo!("Warn here max")
        }

        unsafe {
            asm!("msr cntp_tval_el0, {}", in(reg) time, options(nostack, nomem));
            let flags = 0b11_u64;
            asm!("msr cntp_ctl_el0, {}", in(reg) flags, options(nostack, nomem));

            while {
                let val: u64;
                asm!("mrs {}, cntp_ctl_el0", out(reg) val, options(nostack, nomem));
                (val & 0b100) == 0
            } {}

            let flags = 0_u64;
            asm!("msr cntp_ctl_el0, {}", in(reg) flags, options(nostack, nomem));
        }
    }
}
