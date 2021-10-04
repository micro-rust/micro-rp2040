//! Raw module.
//! Abstract the low level access to registers and peripherals.


#![deny(warnings)]


mod reg;
mod per;

pub use self::reg::{ AtomicRegister, SIORegister };
pub use self::per::{ Peripheral, Single };