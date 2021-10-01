//! System tick peripheral.
//! Simply specifies the Register type to allow for Atomic Hardware accesses.


use crate::raw::AtomicRegister;

use micro::scb;

pub type Systick = scb::Systick<AtomicRegister<u32>>;