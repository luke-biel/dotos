use core::time::Duration;

use tock_registers::{
    interfaces::{ReadWriteable, Writeable},
    register_bitfields,
    register_structs,
    registers::ReadWrite,
};

use crate::{
    arch::arch_impl::statics::TIMER,
    bsp::device_driver::WrappedPointer,
    common::{
        driver::Driver,
        sync::{Mutex, NullLock},
        time_manager::TimeManager,
    },
};

// TODO: Use custom macro
register_bitfields! {
    u32,

    // GPIO Function Select 1
    GPFSEL1 [
        /// Pin 15
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100  // PL011 UART RX

        ],

        /// Pin 14
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100  // PL011 UART TX
        ]
    ],

    /// GPIO Pull-up/down Register
    ///
    /// BCM2837 only.
    GPPUD [
        /// Controls the actuation of the internal pull-up/down control line to ALL the GPIO pins.
        PUD OFFSET(0) NUMBITS(2) [
            Off = 0b00,
            PullDown = 0b01,
            PullUp = 0b10
        ]
    ],

    /// GPIO Pull-up/down Clock Register 0
    ///
    /// BCM2837 only.
    GPPUDCLK0 [
        /// Pin 15
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],

        /// Pin 14
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ]
}

register_structs! {
    RegisterBlock {
        (0x00 => _reserved1),
        (0x04 => gpfsel1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => _reserved2),
        (0x94 => gppud: ReadWrite<u32, GPPUD::Register>),
        (0x98 => gppudclk0: ReadWrite<u32, GPPUDCLK0::Register>),
        (0x9C => @END),
    }
}

pub struct GpioInner {
    registers: WrappedPointer<RegisterBlock>,
}

pub struct Gpio {
    inner: NullLock<GpioInner>,
}

impl GpioInner {
    pub const unsafe fn new(start: usize) -> Self {
        Self {
            registers: WrappedPointer::new(start),
        }
    }

    fn disable_pud_14_15_bcm2837(&mut self) {
        const DELAY: Duration = Duration::from_micros(1);

        self.registers.gppud.write(GPPUD::PUD::Off);
        TIMER.sleep(DELAY);

        self.registers
            .gppudclk0
            .write(GPPUDCLK0::PUDCLK15::AssertClock + GPPUDCLK0::PUDCLK14::AssertClock);
        TIMER.sleep(DELAY);

        self.registers.gppud.write(GPPUD::PUD::Off);
        self.registers.gppudclk0.set(0);
    }

    pub fn map_pl011_uart(&mut self) {
        self.registers
            .gpfsel1
            .modify(GPFSEL1::FSEL15::AltFunc0 + GPFSEL1::FSEL14::AltFunc0);

        self.disable_pud_14_15_bcm2837();
    }
}

impl Gpio {
    pub const unsafe fn new(start: usize) -> Self {
        Self {
            inner: NullLock::new(GpioInner::new(start)),
        }
    }

    pub fn map_pl011_uart(&self) {
        self.inner.map_locked(|inner| inner.map_pl011_uart())
    }
}

impl Driver for Gpio {
    fn compat(&self) -> &'static str {
        "bcm gpio"
    }

    unsafe fn late_init(&self) -> Result<(), &'static str> {
        self.map_pl011_uart();

        Ok(())
    }
}
