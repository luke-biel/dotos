use core::{fmt, marker::PhantomData, ops::Add};

use crate::common::align_down;

pub mod mmu;

pub trait AddressType {}

pub struct Physical;
pub struct Virtual;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address<A: AddressType> {
    addr: usize,
    _phantom: PhantomData<A>,
}

impl AddressType for Physical {}
impl AddressType for Virtual {}

impl<A: AddressType> Address<A> {
    pub const fn new(addr: usize) -> Self {
        Self {
            addr,
            _phantom: PhantomData,
        }
    }

    pub const fn align_down<const T: usize>(self) -> Self {
        Self {
            addr: align_down::<T>(self.addr),
            _phantom: PhantomData,
        }
    }

    pub const fn addr(&self) -> usize {
        self.addr
    }
}

impl<A: AddressType> const From<Address<A>> for usize {
    fn from(item: Address<A>) -> Self {
        item.addr
    }
}

impl fmt::Display for Address<Physical> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let q3: u8 = ((self.addr >> 32) & 0xff) as u8;
        let q2: u16 = ((self.addr >> 16) & 0xffff) as u16;
        let q1: u16 = (self.addr & 0xffff) as u16;

        write!(f, "0x")?;
        write!(f, "{:02x}_", q3)?;
        write!(f, "{:04x}_", q2)?;
        write!(f, "{:04x}", q1)
    }
}

impl fmt::Display for Address<Virtual> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let q4: u16 = ((self.addr >> 48) & 0xffff) as u16;
        let q3: u16 = ((self.addr >> 32) & 0xffff) as u16;
        let q2: u16 = ((self.addr >> 16) & 0xffff) as u16;
        let q1: u16 = (self.addr & 0xffff) as u16;

        write!(f, "0x")?;
        write!(f, "{:04x}_", q4)?;
        write!(f, "{:04x}_", q3)?;
        write!(f, "{:04x}_", q2)?;
        write!(f, "{:04x}", q1)
    }
}

impl<A: AddressType> Add<usize> for Address<A> {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            addr: self.addr + rhs,
            _phantom: PhantomData,
        }
    }
}
