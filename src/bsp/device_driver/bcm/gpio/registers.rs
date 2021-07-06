use tock_registers::registers::ReadWrite;
use tock_registers::{register_bitfields, register_structs};

register_bitfields! {
    u32,

    /// GPIO Function Select 0
    pub GPFSEL0 [
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
    pub GPFSEL1 [
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
