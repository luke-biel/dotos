/// TODO
/// https://s-matyukevich.github.io/raspberry-pi-os/docs/lesson03/rpi-os.html
/// https://github.com/s-matyukevich/raspberry-pi-os/blob/master/src/lesson03/include/peripherals/timer.h
///
/// Create register with base address from gh +
/// impl Driver on SystemTimer +
/// init SystemTime1 irq_handler from driver code +
/// Create scaffolding for system time listeners
/// Connect Scheduler to this

use tock_registers::{register_structs, register_bitfields, registers::{ReadWrite, ReadOnly}};
use crate::bsp::device_driver::WrappedPointer;
use crate::common::memory::mmu::descriptors::MMIODescriptor;
use crate::common::sync::{IRQSafeNullLock, Mutex};
use tock_registers::interfaces::{Readable, Writeable};
use crate::common::driver::Driver;
use crate::common::memory::mmu::map_kernel_mmio;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::common::statics;
use crate::common::exception::asynchronous::{IRQManager, IRQDescriptor, IRQHandler};
use crate::bsp::device_driver::bcm::bcm2xxx_interrupt_controller::{IRQNumber, PeripheralIRQ};

register_bitfields! {
    u32,
    TimerCS [
        TIMER_CS_M0 OFFSET(0) NUMBITS(1) [],
        TIMER_CS_M1 OFFSET(1) NUMBITS(1) [],
        TIMER_CS_M2 OFFSET(2) NUMBITS(1) [],
        TIMER_CS_M3 OFFSET(3) NUMBITS(1) [],
    ]
}

register_structs! {
    pub RegisterBlock {
        (0x00 => timer_cs: ReadWrite<u32, TimerCS::Register>),
        (0x04 => timer_cl0: ReadOnly<u32>),
        (0x08 => _timer_chi),
        (0x0c => _timer_c0),
        (0x10 => timer_c1: ReadWrite<u32>),
        (0x14 => _timer_c2),
        (0x18 => _timer_c3),
        (0x1c => @END),
    }
}

struct SystemTimerInner {
    saved_timer_val: u32,
    registers: WrappedPointer<RegisterBlock>,
}

pub struct SystemTimer {
    descriptor: MMIODescriptor,
    virt_mmio_start_addr: AtomicUsize,
    inner: IRQSafeNullLock<SystemTimerInner>,
}

impl SystemTimerInner {
    const INTERVAL: u32 = 200_000;

    pub const unsafe fn new(addr: usize) -> Self {
        Self {
            saved_timer_val: 0,
            registers: WrappedPointer::new(addr),
        }
    }

    pub unsafe fn init(&mut self, addr: Option<usize>) {
        if let Some(addr) = addr {
            self.registers = WrappedPointer::new(addr);
        }

        self.saved_timer_val = self.registers.timer_cl0.get() + Self::INTERVAL;
        self.registers.timer_c1.set(self.saved_timer_val);
    }

    pub fn handle_irq(&mut self) {
        self.saved_timer_val += Self::INTERVAL;
        self.registers.timer_c1.set(self.saved_timer_val);
        self.registers.timer_cs.write(TimerCS::TIMER_CS_M1::SET);
        crate::info!("Timer interrupt!");
    }
}

impl SystemTimer {
    const IRQ_NUMBER: IRQNumber = IRQNumber::Peripheral(PeripheralIRQ::SystemTimer1);

    pub const unsafe fn new(descriptor: MMIODescriptor) -> Self {
        Self {
            descriptor,
            virt_mmio_start_addr: AtomicUsize::new(0),
            inner: IRQSafeNullLock::new(SystemTimerInner::new(descriptor.start_addr().addr()))
        }
    }
}

impl Driver for SystemTimer {
    fn compat(&self) -> &'static str {
        "bcm system timer"
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        let addr = map_kernel_mmio(self.compat(), self.descriptor)?.addr();

        self.inner.map_locked(|inner| inner.init(Some(addr)));
        self.virt_mmio_start_addr.store(addr, Ordering::Relaxed);

        Ok(())
    }

    fn register_irq_handler(&'static self) -> Result<(), &'static str> {
        statics::INTERRUPT_CONTROLLER.register_handler(Self::IRQ_NUMBER, IRQDescriptor {
            name: self.compat(),
            handler: self
        })?;
        statics::INTERRUPT_CONTROLLER.enable(Self::IRQ_NUMBER);

        Ok(())
    }

    fn virt_mmio_start_addr(&self) -> Option<usize> {
        let addr = self.virt_mmio_start_addr.load(Ordering::Relaxed);

        if addr == 0 {
            None
        } else {
            Some(addr)
        }
    }
}

impl IRQHandler for SystemTimer {
    fn handle(&self) -> Result<(), &'static str> {
        self.inner.map_locked(|inner| inner.handle_irq());

        Ok(())
    }
}
