//! Power and Reset module.


#![deny(warnings)]


mod power;
mod reset;



#[link_section = ".sysbss.POWER"]
#[used]
pub static mut POWER : self::power::PowerSystem = self::power::PowerSystem::empty();


#[link_section = ".sysbss.RESET"]
#[used]
pub static mut RESET : self::reset::ResetSystem = self::reset::ResetSystem::empty();
