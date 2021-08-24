//! Reduced hardware functionality register.

use core::ptr::{
	read_volatile as vread,
	write_volatile as vwrite,
};

use super::Register;

#[repr(transparent)]
pub struct RRegister(u32);


impl Register<u32> for RRegister {
	/// Clears the given bits in the RRegister.
	#[inline(always)]
	fn clear(&mut self, bits: u32) {
		self.0 &= !bits
	}

	/// Sets the given bits in the RRegister.
	#[inline(always)]
	fn set(&mut self, bits: u32) {
		self.0 |= bits
	}


	/// Reads the RRegister from memory.
	#[inline(always)]
	fn read(&self) -> u32 {
		unsafe { vread(self as *const RRegister as *const u32) }
	}

	/// Writes to the RRegister memory location.
	#[inline(always)]
	fn write(&mut self, val: u32) {
		unsafe { vwrite(self as *mut RRegister as *mut u32, val) }
	}
}

impl core::ops::BitAnd<u32> for RRegister {
	type Output = u32;

	#[inline(always)]
	fn bitand(self, rhs: u32) -> u32 {
		self.0 & rhs
	}
}
impl core::ops::BitAndAssign<u32> for RRegister {
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: u32) {
		self.0 &= rhs
	}
}

impl core::ops::BitOr<u32> for RRegister {
	type Output = u32;

	#[inline(always)]
	fn bitor(self, rhs: u32) -> u32 {
		self.0 | rhs
	}
}
impl core::ops::BitOrAssign<u32> for RRegister {
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: u32) {
		self.0 |= rhs
	}
}

impl core::ops::BitXor<u32> for RRegister {
	type Output = u32;

	#[inline(always)]
	fn bitxor(self, rhs: u32) -> u32 {
		self.0 ^ rhs
	}
}
impl core::ops::BitXorAssign<u32> for RRegister {
	#[inline(always)]
	fn bitxor_assign(&mut self, rhs: u32) {
		self.0 ^= rhs
	}
}

impl<X> core::ops::Shl<X> for RRegister
	where u32: core::ops::Shl<X, Output=u32>
{
	type Output = u32;

	#[inline(always)]
	fn shl(self, sh: X) -> u32 {
		self.0 << sh
	}
}

impl<X> core::ops::Shr<X> for RRegister
	where u32: core::ops::Shr<X, Output=u32>
{
	type Output = u32;

	#[inline(always)]
	fn shr(self, sh: X) -> u32 {
		self.0 >> sh
	}
}
