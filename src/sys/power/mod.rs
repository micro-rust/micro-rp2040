//! Power and Reset module.


#![deny(warnings)]


mod power;
mod reset;


pub use self::power::*;
pub use self::reset::*;

#[link_section = ".systembss.POWER"]
#[used]
pub static POWER : self::power::PowerSystem = self::power::PowerSystem::empty();


#[link_section = ".systembss.RESET"]
#[used]
pub static RESET : self::reset::ResetSystem = self::reset::ResetSystem::empty();
