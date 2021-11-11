//! Math module.

#![deny(warnings)]


mod float32;
mod table32;

pub use self::float32::Float32;
pub(crate) use self::table32::SFloatTable;


#[cfg(feature = "bootrom-V2")]
mod float64;
#[cfg(feature = "bootrom-V2")]
mod table64;


#[cfg(feature = "bootrom-V2")]
pub use self::float64::Float64;
#[cfg(feature = "bootrom-V2")]
pub(crate) use self::table64::DFloatTable;


#[link_section = ".systemdata.SFTABLE"]
#[no_mangle]
#[used]
pub static SFTABLE: SFloatTable = SFloatTable::empty();


#[cfg(feature = "bootrom-V2")]
#[link_section = ".systembss.SFTABLE"]
#[used]
pub static DFTABLE: usize = 0x00000000;
