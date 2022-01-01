//! Prelude of the `micro-rp2040` crate.


pub use crate::error::*;

pub use crate::{
	main0, main1,

	hal::gpio::Gpio,

	sync::Spinlock,

	sys::resources::{
		Acquire, Release, Restart,
	},
};



pub(crate) use crate::{
	hal::gpio::GPIOPin,

	raw::{ AtomicRegister, SIORegister },
	
	sync::Syslock,

	sys::resources::{
		AcquireSystem, ReleaseSystem, PauseSystem, ResumeSystem, SystemResource
	},
};



pub(crate) use micro::Register;
