//! Initialization procedures for Core 0.


mod bss;
mod clocks;
mod finish;
mod mpu;
mod reset;
mod sysdata;
mod vectortable;


pub(crate) use self::bss::bss;
pub(crate) use self::clocks::clocks;
pub(crate) use self::finish::finish;
pub(crate) use self::mpu::mpu;
pub(crate) use self::reset::reset;
pub(crate) use self::sysdata::{data, sysdata};
pub(crate) use self::vectortable::vectortable;
