//! Hardware spinlock access and control.
//! Gives access to 2 SpinLock implementations.
//! 
//! # DynSpinLock
//! This SpinLock can be configured/selected at runtime. 
//! 
//! # ConstSpinLock
//! This SpinLock uses const-generics to be configured.


use crate::raw::{ Register, RRegister, addr::SIO };

#[doc(hidden)]
const SPINLOCKS : &'static mut [RRegister; 31] = &mut *((SIO + 0x100) as *mut _);

#[doc(hidden)]
const SYSLOCK : &'static mut RRegister = &mut *((SIO + 0x17C) as *mut _);

/// Dynamic access to the hardware spinlocks.
/// Spinlock 31 is reserved by the framework to do system operations.
pub struct DynSpinLock(usize);

impl DynSpinLock {

	#[inline(always)]
	pub fn acquire(n: usize) -> Option<Self> {
		match SPINLOCKS[n].read() {
			0 => None,
			_ => Some(Self(n))
		}
	}

	#[inline(always)]
	pub unsafe fn release(&mut self) {
		drop(self)
	}
}

impl Drop for DynSpinLock {
	#[inline(always)]
	fn drop(&mut self) {
		SPINLOCKS[self.0].write(0xF)
	}
}

/// Constant access to the hardware spinlocks.
/// SpinLock 31 is reserved by the framework to do system operations.
pub struct ConstSpinLock<const N : usize>;

impl<const N : usize> ConstSpinLock<N> {
	#[inline(always)]
	pub fn acquire() -> Option<Self> {
		match SPINLOCKS[N].read() {
			0 => None,
			_ => Some(Self)
		}
	}

	#[inline(always)]
	pub unsafe fn release(&mut self) {
		drop(self)
	}
}

impl<const N : usize> Drop for ConstSpinLock<N> {
	#[inline(always)]
	fn drop(&mut self) {
		SPINLOCKS[N].write(0xF)
	}
}

#[doc(hidden)]
pub struct SysLock;

impl SysLock {
	#[inline(always)]
	pub fn acquire() -> Option<Self> {
		match SYSLOCK.read() {
			0 => None,
			_ => Some(Self)
		}
	}

	#[inline(always)]
	pub unsafe fn release(&mut self) {
		drop(self)
	}
}

impl Drop for SysLock {
	#[inline(always)]
	fn drop(&mut self) {
		SYSLOCK.write(0xF)
	}
}
