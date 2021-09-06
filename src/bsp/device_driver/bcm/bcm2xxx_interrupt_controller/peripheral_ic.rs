use num_traits::ToPrimitive;
use tock_registers::{
    interfaces::{Readable, Writeable},
    register_structs,
    registers::{ReadOnly, WriteOnly},
};

use crate::{
    bsp::device_driver::{
        bcm::bcm2xxx_interrupt_controller::{PendingIRQs, PeripheralIRQ},
        WrappedPointer,
    },
    common::{
        exception::asynchronous::{IRQContext, IRQDescriptor, IRQManager},
        sync::{IRQSafeNullLock, InitStateLock, Mutex, ReadWriteLock},
    },
    info,
};

register_structs! {
    WriteOnlyRegisterBlock {
        (0x00 => _reserved1),
        (0x10 => enable1: WriteOnly<u32>),
        (0x14 => enable2: WriteOnly<u32>),
        (0x24 => @END),
    },

    ReadOnlyRegisterBlock {
        (0x00 => _reserved1),
        (0x04 => pending1: ReadOnly<u32>),
        (0x08 => pending2: ReadOnly<u32>),
        (0x0c => @END),
    }
}

type WriteOnlyRegisters = WrappedPointer<WriteOnlyRegisterBlock>;
type ReadOnlyRegisters = WrappedPointer<ReadOnlyRegisterBlock>;
type HandlerTable = [Option<(PeripheralIRQ, IRQDescriptor)>; PeripheralIRQ::len()];

pub struct PeripheralInterruptController {
    wo_registers: IRQSafeNullLock<WriteOnlyRegisters>,
    ro_registers: ReadOnlyRegisters,
    handlers: InitStateLock<HandlerTable>,
}

impl PeripheralInterruptController {
    pub const unsafe fn new(mmio_start: usize) -> Self {
        Self {
            wo_registers: IRQSafeNullLock::new(WriteOnlyRegisters::new(mmio_start)),
            ro_registers: ReadOnlyRegisters::new(mmio_start),
            handlers: InitStateLock::new([None; PeripheralIRQ::len()]),
        }
    }

    fn pending(&self) -> PendingIRQs {
        let pending_mask: u64 = (u64::from(self.ro_registers.pending2.get()) << 32)
            | u64::from(self.ro_registers.pending1.get());

        PendingIRQs::new(pending_mask)
    }

    pub fn print_status(&self) {
        info!("  peripheral IC:");
        self.handlers.map_read(|handlers| {
            let mut any = false;
            for handler in handlers.iter() {
                if let Some((irq, descriptor)) = handler {
                    info!(
                        "    {}[{}] -> \"{}\"",
                        irq,
                        irq.to_u64().unwrap(),
                        descriptor.name
                    );
                    any = true;
                }
            }
            if !any {
                info!("    no handlers registered");
            }
        })
    }
}

impl IRQManager for PeripheralInterruptController {
    type IRQNumberT = PeripheralIRQ;

    fn register_handler(
        &self,
        irq: Self::IRQNumberT,
        descriptor: IRQDescriptor,
    ) -> Result<(), &'static str> {
        self.handlers.map_write(|table| {
            let no = irq.to_usize().unwrap();
            if table[no].is_some() {
                return Err("Handler already registered");
            }

            table[no] = Some((irq, descriptor));

            Ok(())
        })
    }

    fn enable(&self, irq: Self::IRQNumberT) {
        self.wo_registers.map_locked(|regs| {
            let no = irq.to_u64().unwrap();
            let reg = if no < 32 {
                &regs.enable1
            } else {
                &regs.enable2
            };
            let enable_bit: u32 = 1 << (no % 32);
            reg.set(enable_bit);
        })
    }

    fn handle_pending<'ctx>(&'ctx self, _token: IRQContext<'ctx>) {
        self.handlers.map_read(|table| {
            self.pending()
                .iter()
                .for_each(|no| match table[no.to_usize().unwrap()] {
                    None => panic!("No handler for IRQ {}", no.to_u64().unwrap()),
                    Some((_, ref d)) => d.handler.handle().expect("Handling IRQ"),
                });
        })
    }
}
