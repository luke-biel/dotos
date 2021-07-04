use crate::bsp::device_driver::WrappedPointer;
use crate::common::driver::DeviceDriver;
use spin::Mutex;
use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};
use tock_registers::{register_bitfields, register_structs};

register_bitfields! {
    u32,

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
        TXFF OFFSET(5) NUMBITS(1) [
            Full = 1,
            Empty = 0
        ],

        /// Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register.
        ///
        /// - If the FIFO is disabled, this bit is set when the receive holding register is empty.
        /// - If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.
        RXFE OFFSET(4) NUMBITS(1) [
            Empty = 1,
            Full = 0
        ],

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
    pub CR [
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

    /// Interrupt Mask Set Clear Register
    pub IMSC [
        OEIM OFFSET(10) NUMBITS(1) [],

        BEIM OFFSET(9) NUMBITS(1) [],

        PEIM OFFSET(8) NUMBITS(1) [],

        FEIM OFFSET(7) NUMBITS(1) [],

        RTIM OFFSET(6) NUMBITS(1) [],

        TXIM OFFSET(5) NUMBITS(1) [],

        RXIM OFFSET(4) NUMBITS(1) [],

        CTSMIM OFFSET(1) NUMBITS(1) []
    ],

    /// Interrupt clear register
    pub ICR [
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

register_structs! {
    pub UARTRegisterBlock {
        (0x0000_0000 => pub dr: ReadWrite<u32>),
        (0x0000_0004 => _reserved1),
        (0x0000_0018 => pub fr: ReadOnly<u32, FR::Register>),
        (0x0000_0024 => pub ibrd: WriteOnly<u32, IBRD::Register>),
        (0x0000_0028 => pub fbrd: WriteOnly<u32, FBRD::Register>),
        (0x0000_002c => pub lcrh: WriteOnly<u32, LCRH::Register>),
        (0x0000_0030 => pub cr: WriteOnly<u32, CR::Register>),
        (0x0000_0034 => _reserved2),
        (0x0000_0038 => pub imsc: ReadWrite<u32, IMSC::Register>),
        (0x0000_003c => _reserved3),
        (0x0000_0044 => pub icr: ReadWrite<u32, ICR::Register>),
        (0x0000_0048 => @END),
    }
}

struct UartInner {
    block: WrappedPointer<UARTRegisterBlock>,
}

pub struct Uart {
    inner: Mutex<UartInner>,
}

impl UartInner {
    const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            block: WrappedPointer::new(mmio_start_addr),
        }
    }

    fn write_blocking(&self, s: &str) {
        for c in s.chars() {
            while self.block.fr.matches_all(FR::TXFF::Full) {}
            self.block.dr.set(c as u32);
        }

        self.flush();
    }

    // fn read_char(&self) -> Option<u8> {
    //     if self.block.fr.matches_all(FR::RXFE::Empty) {
    //         return None;
    //     }
    //
    //     Some(self.block.dr.get() as u8)
    // }

    fn read_char_blocking(&self) -> u8 {
        while self.block.fr.matches_all(FR::RXFE::Empty) {}

        let c = self.block.dr.get() as u8;

        if c == b'\r' {
            b'\n'
        } else {
            c
        }
    }

    // fn clear_rx(&self) {
    //     while self.read_char().is_some() {}
    // }

    fn flush(&self) {
        while self.block.fr.matches_all(FR::BUSY::SET) {}
    }

    fn map_pl011_uart(&self) {
        self.flush();

        self.block.cr.set(0);

        self.block.icr.set(0x7FF);

        self.block.ibrd.write(IBRD::BAUD_DIVINT.val(1));
        self.block.fbrd.write(FBRD::BAUD_DIVFRAC.val(40));

        self.block.lcrh.write(LCRH::FEN::Enabled + LCRH::WLEN::Len8);

        self.block.imsc.set(0x7f1);

        self.block
            .cr
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);
    }
}

impl Uart {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: spin::Mutex::new(UartInner::new(mmio_start_addr)),
        }
    }

    pub fn map_pl011_uart(&self) {
        self.inner.lock().map_pl011_uart()
    }

    pub fn write_blocking(&self, s: &str) {
        self.inner.lock().write_blocking(s)
    }

    // pub fn read_char(&self) -> Option<u8> {
    //     self.inner.lock().read_char()
    // }

    pub fn read_char_blocking(&self) -> u8 {
        self.inner.lock().read_char_blocking()
    }

    // pub fn clear_rx(&self) {
    //     self.inner.lock().clear_rx()
    // }
}

impl DeviceDriver for Uart {
    fn compat(&self) -> &'static str {
        "BCM PL011 UART"
    }

    unsafe fn init(&self) -> Result<(), &'static str> {
        self.map_pl011_uart();

        Ok(())
    }
}
