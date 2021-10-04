//! Abstraction of low level access to registers.


use core::ops::*;


use micro::Register;
use micro::XType;


#[repr(transparent)]
pub struct AtomicRegister<X: XType>(X);


impl<X: XType> Register<X> for AtomicRegister<X> {
	/// Reads the register from memory.
	fn read(&self) -> X {
		unsafe { core::ptr::read_volatile(self.ptr()) }
	}

	/// Write the value to the register.
	fn write(&mut self, val: X) {
		unsafe { core::ptr::write_volatile(self.ptr_mut(), val) }
	}

	/// Clears the given mask in the register.
	fn clear(&mut self, mask: X) {
		unsafe { core::ptr::write_volatile((self as *mut Self as u32 + 0x3000) as *mut _, mask) }
	}

	/// Sets the given mask in the register.
	fn set(&mut self, mask: X) {
		unsafe { core::ptr::write_volatile((self as *mut Self as u32 + 0x2000) as *mut _, mask) }
	}
}


impl<X: XType> BitAnd<X> for AtomicRegister<X> {
	type Output = X;

	fn bitand(self, rhs: X) -> Self::Output {
		self.read() & rhs
	}
}

impl<X: XType> BitXor<X> for AtomicRegister<X> {
	type Output = X;

	fn bitxor(self, rhs: X) -> Self::Output {
		self.read() ^ rhs
	}
}

impl<X: XType> BitOr<X> for AtomicRegister<X> {
	type Output = X;

	fn bitor(self, rhs: X) -> Self::Output {
		self.read() | rhs
	}
}

impl<X: XType> BitAndAssign<X> for AtomicRegister<X> {
	fn bitand_assign(&mut self, rhs: X) {
		unsafe { core::ptr::write_volatile((self as *mut Self as u32 + 0x3000) as *mut _, !rhs) }
	}
}

impl<X: XType> BitXorAssign<X> for AtomicRegister<X> {
	fn bitxor_assign(&mut self, rhs: X) {
		unsafe { core::ptr::write_volatile((self as *mut Self as u32 + 0x1000) as *mut _, rhs) }
	}
}

impl<X: XType> BitOrAssign<X> for AtomicRegister<X> {
	fn bitor_assign(&mut self, rhs: X) {
		unsafe { core::ptr::write_volatile((self as *mut Self as u32 + 0x2000) as *mut _, rhs) }
	}
}


#[repr(transparent)]
pub struct SIORegister<X: XType>(X);


impl<X: XType> Register<X> for SIORegister<X> {
	/// Reads the register from memory.
	fn read(&self) -> X {
		unsafe { core::ptr::read_volatile(self.ptr()) }
	}

	/// Write the value to the register.
	fn write(&mut self, val: X) {
		unsafe { core::ptr::write_volatile(self.ptr_mut(), val) }
	}

	/// Clears the given mask in the register.
	fn clear(&mut self, mask: X) {
		unsafe { core::ptr::write_volatile(self as *mut Self as *mut _, self.read() & !mask) }
	}

	/// Sets the given mask in the register.
	fn set(&mut self, mask: X) {
		unsafe { core::ptr::write_volatile(self as *mut Self as *mut _, self.read() |  mask) }
	}
}


impl<X: XType> BitAnd<X> for SIORegister<X> {
	type Output = X;

	fn bitand(self, rhs: X) -> Self::Output {
		self.read() & rhs
	}
}

impl<X: XType> BitXor<X> for SIORegister<X> {
	type Output = X;

	fn bitxor(self, rhs: X) -> Self::Output {
		self.read() ^ rhs
	}
}

impl<X: XType> BitOr<X> for SIORegister<X> {
	type Output = X;

	fn bitor(self, rhs: X) -> Self::Output {
		self.read() | rhs
	}
}

impl<X: XType> BitAndAssign<X> for SIORegister<X> {
	fn bitand_assign(&mut self, rhs: X) {
		unsafe { core::ptr::write_volatile(self as *mut Self as *mut _, self.read() & rhs) }
	}
}

impl<X: XType> BitXorAssign<X> for SIORegister<X> {
	fn bitxor_assign(&mut self, rhs: X) {
		unsafe { core::ptr::write_volatile(self as *mut Self as *mut _, self.read() ^ rhs) }
	}
}

impl<X: XType> BitOrAssign<X> for SIORegister<X> {
	fn bitor_assign(&mut self, rhs: X) {
		unsafe { core::ptr::write_volatile(self as *mut Self as *mut _, self.read() | rhs) }
	}
}

