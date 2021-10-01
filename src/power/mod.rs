//! Power and Reset module.


#![deny(warnings)]


mod power;
mod reset;


pub use self::power::PowerId;
pub use self::reset::ResetId;

#[link_section = ".systembss.POWER"]
#[used]
pub static mut POWER : self::power::PowerSystem = self::power::PowerSystem::empty();


#[link_section = ".systembss.RESET"]
#[used]
pub static mut RESET : self::reset::ResetSystem = self::reset::ResetSystem::empty();
