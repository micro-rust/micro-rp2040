//! UART Protocol Peripheral.
//! Abstract the hardware UART peripheral.
//! This module acts as a prelude, you can start using the UART by including in your code:
//! ```
//! use rp2040::comm::uart::*;
//! ```



mod config;
mod frame;
mod implementation;




pub use crate::peripherals::pins::uart::*;


pub use self::config::UartConfig;
pub use self::frame::UartFrame;
pub use self::implementation::*;

