use crate::arch::cpu;
use crate::bsp::device_driver::WrappedPointer;
use cfg_if::cfg_if;
use register::{mmio::*, register_bitfields, register_structs};
use tock_registers::registers::Writeable;
use crate::common::driver::DeviceDriver;
use spin::mutex::spin::SpinMutex;

register_bitfields! {
    u32,

    /// GPIO Function Select 0
    GPFSEL0 [
        FSEL0 OFFSET(0) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL1 OFFSET(3) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL2 OFFSET(6) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL3 OFFSET(9) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL4 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL5 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL6 OFFSET(18) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL7 OFFSET(21) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL8 OFFSET(24) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL9 OFFSET(27) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ]
    ],

    /// GPIO Function Select 1
    GPFSEL1 [
        FSEL10 OFFSET(0) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL11 OFFSET(3) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL12 OFFSET(6) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL13 OFFSET(9) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100 // PL011 UART TX
        ],

        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100 // PL011 UART RX
        ],

        FSEL16 OFFSET(18) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL17 OFFSET(21) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL18 OFFSET(24) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ],

        FSEL19 OFFSET(27) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001
        ]
    ],

    /// GPIO Pull-up/down Register
    pub GPPUP [
        /// Controls the actuation of the internal pull-up/down control line to ALL the GPIO pins.
        PUD OFFSET(0) NUMBITS(2) [
            Off = 0b00,
            PullDown = 0b01,
            PullUp = 0b10
        ]
    ],

    /// GPIO Pull-up/down Clock Register 0
    pub GPPUDCLK0 [
        /// Pin 14
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],

        /// Pin 15
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ]
}

register_structs! {
    pub GPIORegisterBlock {
        (0x0000_0000 => pub gpfsel0: ReadWrite<u32, GPFSEL0::Register>),
        (0x0000_0004 => pub gpfsel1: ReadWrite<u32, GPFSEL1::Register>),
        (0x0000_0008 => _reserved1),
        (0x0000_0094 => pub gppup: ReadWrite<u32, GPPUP::Register>),
        (0x0000_0098 => pub gppudclk0: ReadWrite<u32, GPPUDCLK0::Register>),
        (0x0000_00A2 => @END),
    }
}

struct GPIOInner {
    block: WrappedPointer<GPIORegisterBlock>,
}

pub struct GPIO {
    inner: SpinMutex<GPIOInner>,
}

impl GPIOInner {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            block: WrappedPointer::new(mmio_start_addr),
        }
    }

    fn disable_pud(&self) {
        cfg_if! {
            if #[cfg(feature = "board-rpi1")] {
                // bcm2835 impl

                self.block.gppup.write(GPPUP::PUD::Off);
                cpu::sleep(150);
                self.block.gppudclk0.write(GPPUDCLK0::PUDCLK14::AssertClock + GPPUDCLK0::PUDCLK15::AssertClock);
                cpu::sleep(150);
                self.block.gppup.write(GPPUP::PUD::Off);
                self.block.gppudclk0.set(0);
            } else if #[cfg(feature = "board-rpi3")] {
                // bcm2837 impl
                unimplemented!()
            } else if #[cfg(feature = "board-rpi4")] {
                // bcm2711 impl
                unimplemented!()
            }

        }
    }

    pub fn map_pl011_uart(&self) {
        self.block
            .gpfsel1
            .write(GPFSEL1::FSEL15::AltFunc0 + GPFSEL1::FSEL14::AltFunc0);

        self.disable_pud();
    }
}

impl GPIO {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: SpinMutex::new(GPIOInner::new(mmio_start_addr)),
        }
    }

    pub fn map_pl011_uart(&self) {
        self.inner.lock().map_pl011_uart()
    }
}

impl DeviceDriver for GPIO {
    fn compat() -> &'static str {
        "BCM2xxx GPIO"
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        Ok(())
    }
}
