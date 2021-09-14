use derive_more::Display;
use num_derive::{FromPrimitive, ToPrimitive};

use crate::{
    bsp::device_driver::bcm::bcm2xxx_interrupt_controller::peripheral_ic::PeripheralInterruptController,
    common::{
        driver::Driver,
        exception::asynchronous::{IRQContext, IRQDescriptor, IRQManager},
        memory::mmu::descriptors::MMIODescriptor,
    },
    info,
};

mod peripheral_ic;

struct PendingIRQs {
    bitmask: u64,
}

struct PendingIRQsIter {
    bitmask: u64,
}

#[derive(FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum LocalIRQ {}

#[derive(FromPrimitive, ToPrimitive, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Display)]
pub enum PeripheralIRQ {
    SystemTimer1 = 1,
    SystemTimer3 = 3,
    USBController = 9,
    AuxInt = 29,
    I2CSpiSlvInt = 43,
    Pwa0 = 45,
    Pwa1 = 46,
    Smi = 48,
    GPIOInt0 = 49,
    GPIOInt1 = 50,
    GPIOInt2 = 51,
    GPIOInt3 = 52,
    I2CInt = 53,
    SPIInt = 54,
    PCMInt = 55,
    UARTInt = 57,
}

impl PeripheralIRQ {
    pub const fn len() -> usize {
        64
    }
}

#[derive(Copy, Clone)]
pub enum IRQNumber {
    Local(LocalIRQ),
    Peripheral(PeripheralIRQ),
}

pub struct InterruptController {
    peripheral: PeripheralInterruptController,
}

impl PendingIRQs {
    pub fn new(bitmask: u64) -> Self {
        Self { bitmask }
    }

    pub fn iter(&self) -> PendingIRQsIter {
        PendingIRQsIter {
            bitmask: self.bitmask,
        }
    }
}

impl Iterator for PendingIRQsIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.bitmask.trailing_zeros();
        if next == 64 {
            return None;
        }

        self.bitmask &= !(1 << next);

        Some(next as usize)
    }
}

impl InterruptController {
    pub const unsafe fn new(_local_mmio: MMIODescriptor, periph_mmio: MMIODescriptor) -> Self {
        Self {
            peripheral: PeripheralInterruptController::new(periph_mmio),
        }
    }

    pub fn print_status(&self) {
        info!("interrupt controller:");
        self.peripheral.print_status();
        info!("  local IC:");
        info!("    WIP");
    }
}

impl Driver for InterruptController {
    fn compat(&self) -> &'static str {
        "bcm interrupt controller"
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        self.peripheral.init()
    }
}

impl IRQManager for InterruptController {
    type IRQNumberT = IRQNumber;

    fn register_handler(
        &self,
        irq: Self::IRQNumberT,
        descriptor: IRQDescriptor,
    ) -> Result<(), &'static str> {
        match irq {
            IRQNumber::Local(_) => unimplemented!("Local IRQ controller"),
            IRQNumber::Peripheral(irq) => self.peripheral.register_handler(irq, descriptor),
        }
    }

    fn enable(&self, irq: Self::IRQNumberT) {
        match irq {
            IRQNumber::Local(_) => unimplemented!("Local IRQ controller"),
            IRQNumber::Peripheral(irq) => self.peripheral.enable(irq),
        }
    }

    fn handle_pending<'ctx>(&'ctx self, token: IRQContext<'ctx>) {
        self.peripheral.handle_pending(token)
    }
}
