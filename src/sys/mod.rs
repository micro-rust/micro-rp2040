//! System module.
//! Stage 2 bootloader, initialization and Core peripherals.

#![deny(warnings)]


mod boot2;

mod init;


mod ints;

/// RP 2040 Clock configuration.
pub mod clocks;

pub(crate) mod power;


pub(crate) mod resources;


//pub use self::clocks::ClockSystem;
pub use self::clocks::Clocks;
pub use self::ints::InterruptSystem;
pub use self::power::PowerSystem;


#[link_section = ".systembss0.CLOCKS"]
#[used]
pub static mut CLOCKS : [u32; 16] = [0u32; 16];





#[inline(always)]
pub fn coreid() -> u32 {
	unsafe { core::ptr::read_volatile(0xD0000000 as *const u32) }
}
