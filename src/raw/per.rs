//! Low level abstraction of peripehral blocks and single registers.


use core::marker::PhantomData;
use core::ops::*;

use micro::Register;
use micro::XType;


pub struct Peripheral<X: XType, R: Register<X>, const SIZE: usize, const ADDR: u32> {
	_x: PhantomData<X>, _r: PhantomData<R>,
}

impl<X: XType, R: Register<X>, const SIZE: usize, const ADDR: u32> Peripheral<X, R, SIZE, ADDR> {
	/// Const generator.
	#[inline(always)]
	pub const fn get() -> Self {
		Self { _x: PhantomData, _r: PhantomData, }
	}

	/// Const pointer generator.
	#[allow(dead_code)]
	#[inline(always)]
	pub const fn ptr() -> *mut [R; SIZE] {
		ADDR as *mut _
	}
}

impl<X: XType, R: Register<X>, const SIZE: usize, const ADDR: u32> Deref for Peripheral<X, R, SIZE, ADDR> {
	type Target = [R; SIZE];

	fn deref(&self) -> &Self::Target {
		unsafe { & *(ADDR as *const _) }
	}
}

impl<X: XType, R: Register<X>, const SIZE: usize, const ADDR: u32> DerefMut for Peripheral<X, R, SIZE, ADDR> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *(ADDR as *mut _) }
	}
}




pub struct Single<X: XType, R: Register<X>, const ADDR: u32> {
	_x: PhantomData<X>, _r: PhantomData<R>,
}

impl<X: XType, R: Register<X>, const ADDR: u32> Single<X, R, ADDR> {
	/// Const generator.
	#[inline(always)]
	pub const fn get() -> Self {
		Self { _x: PhantomData, _r: PhantomData, }
	}

	/// Const pointer generator.
	#[allow(dead_code)]
	#[inline(always)]
	pub const fn ptr() -> *mut R {
		ADDR as *mut _
	}
}

impl<X: XType, R: Register<X>, const ADDR: u32> Deref for Single<X, R, ADDR> {
	type Target = R;

	fn deref(&self) -> &Self::Target {
		unsafe { & *(ADDR as *const _) }
	}
}

impl<X: XType, R: Register<X>, const ADDR: u32> DerefMut for Single<X, R, ADDR> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *(ADDR as *mut _) }
	}
}

