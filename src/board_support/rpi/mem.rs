#![allow(dead_code)]

use core::cell::UnsafeCell;
use register::{register_structs, register_bitfields, mmio::*};

use crate::arch::IntPtr;
use crate::pointer_iter::PointerIter;

extern "Rust" {
    static __bss_start: UnsafeCell<IntPtr>;
    static __bss_end: UnsafeCell<IntPtr>;
}

// GPIO registers
//
// Spec https://www.raspberrypi.org/documentation/hardware/raspberrypi/bcm2835/BCM2835-ARM-Peripherals.pdf
register_bitfields! {
    u32,

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
    ],

    // Flag register
    pub FR [
        /// Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the
        /// Line Control Register, LCR_H.
        ///
        /// - If the FIFO is disabled, this bit is set when the transmit holding register is empty.
        /// - If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty.
        /// - This bit does not indicate if there is data in the transmit shift register.
        TXFE OFFSET(7) NUMBITS(1) [],

        /// Receive FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_ LCRH Register.
        ///
        /// - If the FIFO is disabled, this bit is set when the receive holding register is full.
        /// - If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full.
        RXFF OFFSET(6) NUMBITS(1) [],

        /// Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the
        /// LCR_H Register.
        ///
        /// - If the FIFO is disabled, this bit is set when the transmit holding register is full.
        /// - If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.
        TXFF OFFSET(5) NUMBITS(1) [],

        /// Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register.
        ///
        /// - If the FIFO is disabled, this bit is set when the receive holding register is empty.
        /// - If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.
        RXFE OFFSET(4) NUMBITS(1) [],

        /// UART busy.
        /// - If this bit is set to 1, the UART is busy transmitting data.
        ///
        /// This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register.
        /// This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not.
        BUSY OFFSET(3) NUMBITS(1) [],

        /// Clear to send.
        /// This bit is the complement of the UART clear to send, nUARTCTS, modem status input.
        /// That is, the bit is 1 when nUARTCTS is LOW.
        CTS OFFSET(1) NUMBITS(1) []
    ],


    /// Integer Baud Rate Divisor.
    pub IBRD [
        BAUD_DIVINT OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional Baud Rate Divisor.
    pub FBRD [
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) []
    ],

    /// Line control register
    pub LCRH [
        /// Stick parity select
        /// 0 = stick parity is disabled
        /// 1 = either:
        ///     - if the EPS bit is 0 then the parity bit is transmitted and checked as a 1
        ///     - if the EPS bit is 1 then the parity bit is transmitted and checked as a 0.
        SPS OFFSET(7) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Word length
        WLEN OFFSET(5) NUMBITS(2) [
            Len8 = 0b11,
            Len7 = 0b10,
            Len6 = 0b01,
            Len5 = 0b00
        ],

        /// Enable Fifos
        FEN OFFSET(4) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Two stop bits select
        STP2 OFFSET(3) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Even parity select
        EPS OFFSET(2) NUMBITS(1) [
            Odd = 0,
            Even = 1
        ],

        /// Parity enabled
        PEN OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Send break
        BRK OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Control registrer
    CR [
        /// CTS hardware flow control enable.
        CTSEN OFFSET(15) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// RTS hardware flow control enable.
        RTSEN OFFSET(14) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Request to send
        RTS OFFSET(11) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Receive enabled
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Transmit enabled
        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Loopback enabled
        LBE OFFSET(7) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// UART enabled
        UARTEN OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Interrupt clear register
    ICR [
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

register_structs! {
    pub GPIORegisterBlock {
        (0x0000_0000 => _gpio_base), // RPI1 = 0x2020_0000
        (0x0000_0094 => pub gppup: ReadWrite<u32, GPPUP::Register>),
        (0x0000_0098 => pub gppudclk0: ReadWrite<u32, GPPUDCLK0::Register>),
        (0x0000_00A2 => @END),
    },

    pub UARTRegisterBlock {
        (0x0000_0000 => pub dr: ReadWrite<u32>), // RPI1 = 0x2020_1000
        (0x0000_0004 => _reserved1),
        (0x0000_0018 => pub fr: ReadOnly<u32, FR::Register>),
        (0x0000_0024 => pub ibrd: WriteOnly<u32, IBRD::Register>),
        (0x0000_0028 => pub fbrd: WriteOnly<u32, FBRD::Register>),
        (0x0000_002c => pub lcrh: WriteOnly<u32, LCRH::Register>),
        (0x0000_0030 => pub cr: WriteOnly<u32, CR::Register>),
        (0x0000_0034 => _reserved2),
        (0x0000_0044 => pub icr: ReadWrite<u32, ICR::Register>),
        (0x0000_0048 => @END),
    }
}

pub struct GPIO {
    registers: GPIORegisterBlock,
}

// impl GPIO {
//     pub fn new() -> Self {
//
//     }
// }

pub const MMIO_BASE: u32 = 0x2000_0000;

pub const GPIO_BASE: u32 = MMIO_BASE + 0x20_0000;

pub const GPPUD: u32 = GPIO_BASE + 0x94;

pub const GPPUDCLK0: u32 = GPIO_BASE + 0x98;

pub const UART0_BASE: u32 = GPIO_BASE + 0x1000; // 0x2020_1000

pub const UART0_DR: u32 = UART0_BASE + 0x00;
pub const UART0_RSRECR: u32 = UART0_BASE + 0x04;
pub const UART0_FR: u32 = UART0_BASE + 0x18;
pub const UART0_ILPR: u32 = UART0_BASE + 0x20;
pub const UART0_IBRD: u32 = UART0_BASE + 0x24;
pub const UART0_FBRD: u32 = UART0_BASE + 0x28;
pub const UART0_LCRH: u32 = UART0_BASE + 0x2C;
pub const UART0_CR: u32 = UART0_BASE + 0x30;
pub const UART0_IFLS: u32 = UART0_BASE + 0x34;
pub const UART0_IMSC: u32 = UART0_BASE + 0x38;
pub const UART0_RIS: u32 = UART0_BASE + 0x3C;
pub const UART0_MIS: u32 = UART0_BASE + 0x40;
pub const UART0_ICR: u32 = UART0_BASE + 0x44;
pub const UART0_DMACR: u32 = UART0_BASE + 0x48;
pub const UART0_ITCR: u32 = UART0_BASE + 0x80;
pub const UART0_ITIP: u32 = UART0_BASE + 0x84;
pub const UART0_ITOP: u32 = UART0_BASE + 0x88;
pub const UART0_TDR: u32 = UART0_BASE + 0x8C;

pub const MBOX_BASE: u32 = MMIO_BASE + 0xB880;
pub const MBOX_READ: u32 = MBOX_BASE + 0x00;
pub const MBOX_STATUS: u32 = MBOX_BASE + 0x18;
pub const MBOX_WRITE: u32 = MBOX_BASE + 0x20;

pub fn bss_section() -> PointerIter<IntPtr> {
    unsafe { PointerIter::new(__bss_start.get(), __bss_end.get()) }
}
