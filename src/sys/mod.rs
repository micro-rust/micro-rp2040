//! System module.
//! Stage 2 bootloader, initialization and Core peripherals.

#![deny(warnings)]


mod boot2;

mod init;


mod ints;

/// RP 2040 Clock configuration.
pub mod clocks;

pub(crate) mod power;


//pub use self::clocks::ClockSystem;
pub use self::clocks::Clocks;
pub use self::ints::InterruptSystem;
pub use self::power::PowerSystem;


/// Resources of the RP2040.
#[link_section = ".systembss.RESOURCES"]
pub(crate) static mut RESOURCES : [u32; 8] = [0u32; 8];


#[link_section = ".systembss.CLOCKS"]
#[used]
pub static mut CLOCKS : Clocks = Clocks::empty();


#[link_section = ".systembss.TESTCLOCKS"]
#[used]
pub static mut TESTCLOCKS : [u32; 16] = [0u32; 16];



pub trait SystemResource: Sized {
    /// Function to acquire the resource.
    fn acquire() -> Result<Self, crate::error::SystemError>;
}



#[inline(always)]
pub fn coreid() -> u32 {
	unsafe { core::ptr::read_volatile(0xD0000000 as *const u32) }
}
