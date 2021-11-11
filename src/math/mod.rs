//! Math module.

#![deny(warnings)]


// 32-bit float.
mod float32;
mod table32;


pub use self::float32::Float32;
pub(crate) use self::table32::SFloatTable;


#[link_section = ".systemdata.SFTABLE"]
#[used]
pub static SFTABLE: SFloatTable = SFloatTable::empty();




// 64-bit float.
#[cfg(feature = "bootrom-V2")]
mod float64;
#[cfg(feature = "bootrom-V2")]
mod table64;


#[cfg(feature = "bootrom-V2")]
pub use self::float64::Float64;
#[cfg(feature = "bootrom-V2")]
pub(crate) use self::table64::DFloatTable;


#[cfg(feature = "bootrom-V2")]
#[link_section = ".systembss.SFTABLE"]
#[used]
pub static DFTABLE: usize = 0x00000000;




// 32-bit integer (signed and unsigned).
mod int32;
mod uint32;

pub use self::int32::Int32;
pub use self::uint32::UInt32;