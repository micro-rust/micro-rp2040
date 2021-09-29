//! System Lock synchronization.
//! Reserves Spinlock 31 as system lock, which is used to moderate concurrent
//! access to shared resources of the RP2040.


use crate::raw::Single;
use crate::raw::SIORegister;


use micro::Register;


static mut LOCK : Single<u32, SIORegister<u32>, 0xD000017C> = Single::get();


pub struct Syslock;

impl Syslock {
	/// Acquires the lock if it's available.
	#[inline(always)]
	pub fn acquire() -> Option<Self> {
		match unsafe { LOCK.read() } {
			0 => None,
			_ => Some(Self),
		}
	}

	/// Releases the Syslock.
	#[inline(always)]
	pub fn release(&self) {
		unsafe { LOCK.write(1); }
	}
}

impl Drop for Syslock {
	fn drop(&mut self) {
		unsafe { LOCK.write(1); }
	}
}
