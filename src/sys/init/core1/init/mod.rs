//! Initialization procedures for Core 1.


mod mpu;
mod romfunc;
mod sysdata;
mod vectortable;


pub(crate) use self::mpu::mpu;
pub(crate) use self::romfunc::romfunc;
pub(crate) use self::sysdata::sysdata;
pub(crate) use self::vectortable::vectortable;
