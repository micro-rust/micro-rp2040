//! Time module.
//! Contains all abstractions for timers, alarms, PWM, System tick and RTC.


#![deny(warnings)]


#[link_section = ".systembss.CLOCKS"]
#[used]
pub static mut CLOCKS : Clocks = Clocks::empty();


/// RP 2040 Clock configuration.
pub mod clocks;

pub use self::clocks::Clocks;

/// ARM Cortex M0+ System tick peripheral.
mod systick;

pub use self::systick::Systick;

pub mod alarm;
pub mod rtc;
pub mod timer;
pub mod watchdog;
