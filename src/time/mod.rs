//! Time module.
//! Contains all abstractions for timers, alarms, PWM, System tick and RTC.


#![deny(warnings)]



/// ARM Cortex M0+ System tick peripheral.
mod systick;

pub use self::systick::Systick;

pub mod alarm;
pub mod rtc;
pub mod timer;
pub mod watchdog;
