//! Extended hardware functionality register.


use core::ptr::{
	read_volatile as vread,
	write_volatile as vwrite,
};

use super::Register;

#[repr(transparent)]
pub struct XRegister(u32);

impl Register<u32> for XRegister {
	/// Clears the given bits in the XRegister.
	#[inline(always)]
	fn clear(&mut self, bits: u32) {
		unsafe { vwrite((self as *mut XRegister as u32 + 0x3000) as *mut u32, bits) }
	}

	/// Sets the given bits in the XRegister.
	#[inline(always)]
	fn set(&mut self, bits: u32) {
		unsafe { vwrite((self as *mut XRegister as u32 + 0x2000) as *mut u32, bits) }
	}


	/// Reads the XRegister from memory.
	#[inline(always)]
	fn read(&self) -> u32 {
		unsafe { vread(self as *const XRegister as *const u32) }
	}

	/// Writes to the XRegister memory location.
	#[inline(always)]
	fn write(&mut self, val: u32) {
		unsafe { vwrite(self as *mut XRegister as *mut u32, val) }
	}
}

impl core::ops::BitAnd<u32> for XRegister {
	type Output = u32;

	#[inline(always)]
	fn bitand(self, rhs: u32) -> u32 {
		self.0 & rhs
	}
}
impl core::ops::BitAndAssign<u32> for XRegister {
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: u32) {
		unsafe { vwrite((self as *mut XRegister as u32 + 0x3000) as *mut u32, !rhs) }
	}
}

impl core::ops::BitOr<u32> for XRegister {
	type Output = u32;

	#[inline(always)]
	fn bitor(self, rhs: u32) -> u32 {
		self.0 | rhs
	}
}
impl core::ops::BitOrAssign<u32> for XRegister {
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: u32) {
		unsafe { vwrite((self as *mut XRegister as u32 + 0x2000) as *mut u32, rhs) }
	}
}

impl core::ops::BitXor<u32> for XRegister {
	type Output = u32;

	#[inline(always)]
	fn bitxor(self, rhs: u32) -> u32 {
		self.0 ^ rhs
	}
}
impl core::ops::BitXorAssign<u32> for XRegister {
	#[inline(always)]
	fn bitxor_assign(&mut self, rhs: u32) {
		unsafe { vwrite((self as *mut XRegister as u32 + 0x1000) as *mut u32, rhs) }
	}
}

impl<X> core::ops::Shl<X> for XRegister
	where u32: core::ops::Shl<X, Output=u32>
{
	type Output = u32;

	#[inline(always)]
	fn shl(self, sh: X) -> u32 {
		self.0 << sh
	}
}

impl<X> core::ops::Shr<X> for XRegister
	where u32: core::ops::Shr<X, Output=u32>
{
	type Output = u32;

	#[inline(always)]
	fn shr(self, sh: X) -> u32 {
		self.0 >> sh
	}
}
