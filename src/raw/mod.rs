//! Raw access to the RP2040.

pub mod addr;

mod reg;

pub(crate) use self::reg::{ Register, RRegister, XRegister };