//! Common functionality for Core 0 and Core 1 initialization.


#![deny(warnings)]


mod core0;
mod core1;



pub(self) unsafe fn load(mut s: *mut u32, e: *mut u32, mut l: *const u32) {
	use core::ptr::{
		read_volatile as read,
		write_volatile as write,
	};

	if s >= e { return; }

	while s < e {
		write(s, read(l));
		s = s.offset(1);
		l = l.offset(1);
	}
}

pub(self) unsafe fn zero(mut s: *mut u32, e: *mut u32) {
	use core::ptr::write_volatile as write;

	if s >= e { return; }

	while s < e {
		write(s, 0u32);
		s = s.offset(1);
	}
}
