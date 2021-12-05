//! Power and Reset module.


#![deny(warnings)]


mod power;
mod reset;


pub use self::power::*;
pub use self::reset::*;


#[link_section = ".sysbss0.RESET"]
#[used]
pub static RESET : self::reset::ResetSystem = self::reset::ResetSystem::empty();
