//! Power subsystem.
//! Allows for power management, powering on and off different parts of the
//! RP2040 and control of voltage levels and brownout detection.


#![allow(non_camel_case_types)]


use crate::raw::AtomicRegister;
use crate::sync::Syslock;

use micro::{ Peripheral, Register };


type PSM = Peripheral<u32, AtomicRegister<u32>, 4, 0x40010000>;

type VREG = Peripheral<u32, AtomicRegister<u32>, 3, 0x40064000>;



pub struct PowerSystem;

impl PowerSystem {
	/// Static initializer.
	pub const fn empty() -> Self {
		PowerSystem
	}

	/// Overvolts the RP2040 to the given non-nominal voltage.
	#[inline]
	pub unsafe fn overvolt(&self, v: Overvoltage) -> Result<(), ()> {
		const MASK: u32 = 0xF << 4;

		let mut VREG: VREG = Peripheral::get();

		let val = (VREG[0].read() & MASK) | (u32::from(v) << 4);

		match Syslock::acquire() {
			Some(_) => Ok( VREG[0].write( val ) ),
			_ => Err(()),
		}
	}


	/// Undervolts the RP2040 to the given non-nominal voltage.
	#[inline]
	pub unsafe fn undervolt(&self, v: Undervoltage) -> Result<(), ()> {
		const MASK: u32 = 0xF << 4;

		let mut VREG: VREG = Peripheral::get();

		let val = (VREG[0].read() & MASK) | (u32::from(v) << 4);

		match Syslock::acquire() {
			Some(_) => Ok( VREG[0].write( val ) ),
			_ => Err(()),
		}
	}

	/// Sets the nominal voltage.
	#[inline]
	pub unsafe fn nominal(&self) {
		const MASK: u32 = 0xF << 4;

		let mut VREG: VREG = Peripheral::get();

		let val = (VREG[0].read() & MASK) | (0b1011 << 4);

		VREG[0].write( val )
	}

	/// Sets the given borwnout level.
	#[inline]
	pub unsafe fn brownout(&self, level: Option<BrownoutLevel>) -> Result<(), ()> {
		let mut VREG: VREG = Peripheral::get();

		match Syslock::acquire() {
			Some(_) => match level {
				None => { Ok( VREG[1].clear(1) ) },
				Some(l) => Ok( VREG[1].write( u32::from(l) | 1 ) ),
			},
			_ => Err(()),
		}
	}

	/// Powers on the given domain.
	#[inline]
	pub fn poweron(&self, id: PowerId) {
		let mut PSM: PSM = Peripheral::get();

		PSM[1].clear(u32::from(id));
		PSM[0].set(u32::from(id))
	}

	/// Powers off the given domain.
	#[inline]
	pub unsafe fn poweroff(&self, id: PowerId) -> Result<(), ()> {
		let mut PSM: PSM = Peripheral::get();

		match Syslock::acquire() {
			Some(_) => {
				PSM[0].clear(u32::from(id));
				PSM[1].set(u32::from(id));

				Ok(())
			},

			_ => Err(()),
		}
	}

	/// Indicates if the given domain is powered on.
	/// This can return `false` for a while until the whole domain wakes up.
	#[inline]
	pub fn enabled(&self, id: PowerId) -> bool {
		let PSM: PSM = Peripheral::get();
		( PSM[3].read() & u32::from(id) ) == u32::from(id)
	}

	/// Indicates if the given domain is powered off.
	/// This can return `true` for a while until the whole domain wakes up.
	#[inline]
	pub fn disabled(&self, id: PowerId) -> bool {
		let PSM: PSM = Peripheral::get();
		( PSM[3].read() & u32::from(id) ) != u32::from(id)
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Overvoltage {
	MilliVolts_1150 = 0b1100,
	MilliVolts_1200 = 0b1101,
	MilliVolts_1250 = 0b1110,
	MilliVolts_1300 = 0b1111,
}

impl core::convert::From<Overvoltage> for u32 {
	fn from(b: Overvoltage) -> u32 {
		b as u8 as u32
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Undervoltage {
	MilliVolts_0800 = 0b0101,
	MilliVolts_0850 = 0b0110,
	MilliVolts_0900 = 0b0111,
	MilliVolts_0950 = 0b1000,
	MilliVolts_1000 = 0b1001,
	MilliVolts_1050 = 0b1010,
}

impl core::convert::From<Undervoltage> for u32 {
	fn from(b: Undervoltage) -> u32 {
		b as u8 as u32
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BrownoutLevel {
	MilliVolts_0473 = 0b0000,
	MilliVolts_0516 = 0b0001,
	MilliVolts_0559 = 0b0010,
	MilliVolts_0602 = 0b0011,
	MilliVolts_0645 = 0b0100,
	MilliVolts_0688 = 0b0101,
	MilliVolts_0731 = 0b0110,
	MilliVolts_0774 = 0b0111,

	MilliVolts_0817 = 0b1000,
	MilliVolts_0860 = 0b1001,
	MilliVolts_0903 = 0b1010,
	MilliVolts_0946 = 0b1011,
	MilliVolts_0989 = 0b1100,
	MilliVolts_1032 = 0b1101,
	MilliVolts_1075 = 0b1110,
	MilliVolts_1118 = 0b1111,
}

impl core::convert::From<BrownoutLevel> for u32 {
	fn from(b: BrownoutLevel) -> u32 {
		b as u8 as u32
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PowerId(u32);

impl PowerId {
	pub const ROSC : PowerId = PowerId(1 << 0);

	pub const XOSC : PowerId = PowerId(1 << 1);

	pub const CLOCKS : PowerId = PowerId(1 << 2);

	pub const RESETS : PowerId = PowerId(1 << 3);

	pub const BUSFRABIC : PowerId = PowerId(1 << 4);

	pub const ROM : PowerId = PowerId(1 << 5);

	pub const SRAM0 : PowerId = PowerId(1 << 6);
	pub const SRAM1 : PowerId = PowerId(1 << 7);
	pub const SRAM2 : PowerId = PowerId(1 << 8);
	pub const SRAM3 : PowerId = PowerId(1 << 9);
	pub const SRAM4 : PowerId = PowerId(1 << 10);
	pub const SRAM5 : PowerId = PowerId(1 << 11);

	pub const XIP : PowerId = PowerId(1 << 12);

	pub const VREG : PowerId = PowerId(1 << 13);

	pub const SIO : PowerId = PowerId(1 << 14);

	pub const PROC0 : PowerId = PowerId(1 << 15);
	pub const PROC1 : PowerId = PowerId(1 << 16);

	/// XORs the ID.
	pub const fn inverse(&self) -> PowerId {
		PowerId(self.0 ^ 0xFFFFFFFF)
	}
}

impl core::ops::Add<Self> for PowerId {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		PowerId(self.0 | rhs.0)
	}
}

impl core::ops::BitOr<Self> for PowerId {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output {
		PowerId(self.0 | rhs.0)
	}
}

impl core::ops::AddAssign<Self> for PowerId {
	fn add_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0
	}
}

impl core::ops::BitOrAssign<Self> for PowerId {
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0
	}
}

impl core::convert::From<PowerId> for u32 {
	fn from(x: PowerId) -> u32 {
		x.0
	}
}
