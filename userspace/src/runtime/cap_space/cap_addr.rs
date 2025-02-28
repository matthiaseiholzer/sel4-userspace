use core::clone::Clone;
use core::cmp::Eq;
use core::cmp::PartialEq;
use core::fmt::Debug;
use core::ops::Add;
use core::prelude::rust_2024::derive;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CapAddr {
    pub addr: usize,
    pub depth: u8,
}

impl CapAddr {
    pub fn from(value: usize, depth: u8) -> Self {
        CapAddr { addr: value, depth }
    }
}

impl CapAddr {
    pub const fn from_const(value: usize) -> Self {
        CapAddr {
            addr: value,
            depth: 64,
        }
    }

    pub const fn null() -> Self {
        CapAddr::from_const(0)
    }
}

impl Add<usize> for CapAddr {
    type Output = CapAddr;
    fn add(mut self, rhs: usize) -> Self::Output {
        self.addr += rhs;

        self
    }
}
