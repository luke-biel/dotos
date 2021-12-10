use core::{
    fmt,
    fmt::Arguments,
    sync::atomic::{AtomicUsize, Ordering},
};

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields,
    register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

use crate::{
    arch::arch_impl::cpu::instructions::nop,
    bsp::device_driver::{
        bcm::bcm2xxx_interrupt_controller::{IRQNumber, PeripheralIRQ},
        WrappedPointer,
    },
    common::{
        driver::Driver,
        exception::asynchronous::{IRQDescriptor, IRQHandler, IRQManager},
        memory::mmu::{descriptors::MMIODescriptor, map_kernel_mmio},
        serial_console,
        statics,
        sync::{IRQSafeNullLock, Mutex},
    },
};

register_bitfields! {
    u32,

    FR [
        TXFE OFFSET(7) NUMBITS(1) [],
        TXFF OFFSET(5) NUMBITS(1) [],
        RXFE OFFSET(4) NUMBITS(1) [],
        BUSY OFFSET(3) NUMBITS(1) []
    ],

    IBRD [
        BAUD_DIVINT OFFSET(0) NUMBITS(16) []
    ],

    FBRD [
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) []
    ],

    LCR_H [
        WLEN OFFSET(5) NUMBITS(2) [
            Five = 0b00,
            Six = 0b01,
            Seven = 0b10,
            Eight = 0b11
        ],

        FEN  OFFSET(4) NUMBITS(1) [
            FifosDisabled = 0,
            FifosEnabled = 1
        ]
    ],

    CR [
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        UARTEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    IFLS [
        RXIFLSEL OFFSET(3) NUMBITS(5) [
            OneEigth = 0b000,
            OneQuarter = 0b001,
            OneHalf = 0b010,
            ThreeQuarters = 0b011,
            SevenEights = 0b100
        ]
    ],

    IMSC [
        RTIM OFFSET(6) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],
        RXIM OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    MIS [
        RTMIS OFFSET(6) NUMBITS(1) [],
        RXMIS OFFSET(4) NUMBITS(1) []
    ],

    ICR [
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => dr: ReadWrite<u32>),
        (0x04 => _reserved1),
        (0x18 => fr: ReadOnly<u32, FR::Register>),
        (0x1c => _reserved2),
        (0x24 => ibrd: WriteOnly<u32, IBRD::Register>),
        (0x28 => fbrd: WriteOnly<u32, FBRD::Register>),
        (0x2c => lcr_h: WriteOnly<u32, LCR_H::Register>),
        (0x30 => cr: WriteOnly<u32, CR::Register>),
        (0x34 => ifls: ReadWrite<u32, IFLS::Register>),
        (0x38 => imsc: ReadWrite<u32, IMSC::Register>),
        (0x3c => _reserved3),
        (0x40 => mis: ReadOnly<u32, MIS::Register>),
        (0x44 => icr: WriteOnly<u32, ICR::Register>),
        (0x48 => @END),
    }
}

pub struct PL011UartInner {
    registers: WrappedPointer<RegisterBlock>,
}

pub struct PL011Uart {
    mmio_descriptor: MMIODescriptor,
    virt_mmio_start_addr: AtomicUsize,
    inner: IRQSafeNullLock<PL011UartInner>,
}

impl PL011UartInner {
    pub const unsafe fn new(start: usize) -> Self {
        Self {
            registers: WrappedPointer::new(start),
        }
    }

    pub fn init(&mut self, new_mmio_start_addr: Option<usize>) -> Result<(), &'static str> {
        if let Some(addr) = new_mmio_start_addr {
            unsafe {
                self.registers = WrappedPointer::new(addr);
            }
        }

        self.flush();

        self.registers.cr.set(0);
        self.registers.icr.write(ICR::ALL::CLEAR);

        self.registers.ibrd.write(IBRD::BAUD_DIVINT.val(26));
        self.registers.fbrd.write(FBRD::BAUD_DIVFRAC.val(3));

        self.registers
            .lcr_h
            .write(LCR_H::FEN::FifosEnabled + LCR_H::WLEN::Eight);

        self.registers.ifls.write(IFLS::RXIFLSEL::OneEigth);
        self.registers
            .imsc
            .write(IMSC::RXIM::Enabled + IMSC::RTIM::Enabled);

        self.registers
            .cr
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);

        Ok(())
    }

    fn flush(&self) {
        while self.registers.fr.matches_all(FR::BUSY::SET) {
            unsafe { nop() }
        }
    }

    fn wait_for_tx_fifo(&self) {
        while self.registers.fr.matches_all(FR::TXFF::SET) {
            unsafe { nop() }
        }
    }

    fn wait_for_rx_fifo(&self) {
        while self.registers.fr.matches_all(FR::RXFE::SET) {
            unsafe { nop() }
        }
    }

    fn write_char(&mut self, c: char) {
        self.wait_for_tx_fifo();

        self.registers.dr.set(c as u32)
    }

    fn read_char(&mut self, block: bool) -> Option<char> {
        if self.registers.fr.matches_all(FR::RXFE::SET) {
            if !block {
                return None;
            }

            self.wait_for_rx_fifo()
        }

        let res = self.registers.dr.get() as u8 as char;

        if res == '\r' {
            Some('\n')
        } else {
            Some(res)
        }
    }
}

impl fmt::Write for PL011UartInner {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.write_char(c)
        }

        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.write_char(c);

        Ok(())
    }
}

impl PL011Uart {
    const IRQ_NUMBER: IRQNumber = IRQNumber::Peripheral(PeripheralIRQ::UARTInt);

    pub const unsafe fn new(mmio_descriptor: MMIODescriptor) -> Self {
        Self {
            mmio_descriptor,
            virt_mmio_start_addr: AtomicUsize::new(0),
            inner: IRQSafeNullLock::new(PL011UartInner::new(mmio_descriptor.start_addr().addr())),
        }
    }
}

impl Driver for PL011Uart {
    fn compat(&self) -> &'static str {
        "bcm pl011 uart"
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        let addr = map_kernel_mmio(self.compat(), self.mmio_descriptor)?;

        self.inner
            .map_locked(|inner| inner.init(Some(addr.addr())))?;

        self.virt_mmio_start_addr
            .store(addr.addr(), Ordering::Relaxed);

        Ok(())
    }

    fn register_irq_handler(&'static self) -> Result<(), &'static str> {
        statics::INTERRUPT_CONTROLLER.register_handler(
            Self::IRQ_NUMBER,
            IRQDescriptor {
                name: self.compat(),
                handler: self,
            },
        )?;
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

impl serial_console::Write for PL011Uart {
    fn write_char(&self, c: char) {
        self.inner.map_locked(|inner| inner.write_char(c))
    }

    fn write_fmt(&self, args: Arguments) -> fmt::Result {
        self.inner
            .map_locked(|inner| fmt::Write::write_fmt(inner, args))
    }

    fn flush(&self) {
        self.inner.map_locked(|inner| inner.flush())
    }
}

impl serial_console::Read for PL011Uart {
    fn read_char(&self) -> char {
        self.inner
            .map_locked(|inner| inner.read_char(true).expect("inner read_char"))
    }

    fn clear(&self) {
        while self
            .inner
            .map_locked(|inner| inner.read_char(false))
            .is_some()
        {}
    }
}

impl fmt::Write for PL011Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            serial_console::Write::write_char(self, c);
        }

        Ok(())
    }
}

impl IRQHandler for PL011Uart {
    fn handle(&self) -> Result<(), &'static str> {
        self.inner.map_locked(|inner| {
            let pending = inner.registers.mis.extract();

            inner.registers.icr.write(ICR::ALL::CLEAR);

            if pending.matches_any(MIS::RXMIS::SET + MIS::RTMIS::SET) {
                while inner.read_char(false).is_some() {}
                // while let Some(c) = inner.read_char(false) {
                //    inner.write_char(c)
                // }
            }
        });

        Ok(())
    }
}
