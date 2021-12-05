//! Prelude of the `micro-rp2040` crate.


pub use crate::{ main0, main1 };

pub use crate::error::*;

pub use crate::sys::resources::SystemResource;

pub use crate::peripherals::pins::Gpio;


pub(crate) use crate::raw::{ AtomicRegister, SIORegister };

pub(crate) use crate::sys::resources::{ Resources, DropResources };

pub(crate) use crate::sync::{ Syslock, Spinlock };



pub(crate) use micro::Register;
