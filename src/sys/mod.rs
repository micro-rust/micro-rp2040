//! System module.
//! Stage 2 bootloader, initialization and Core peripherals.

#![deny(warnings)]


mod boot2;

mod init;

mod res;



pub use self::res::SystemResource;
pub use self::res::Resource;


pub(crate) use self::res::RESOURCES;



#[inline(always)]
pub fn coreid() -> u32 {
	unsafe { core::ptr::read_volatile(0xD0000000 as *const u32) }
}
