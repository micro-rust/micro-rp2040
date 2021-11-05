//! UART Protocol Peripheral.
//! Abstract the hardware UART peripheral.
//! This module acts as a prelude, you can start using the UART by including in your code:
//! ```
//! use rp2040::comm::uart::*;
//! ```



mod config;
mod frame;
mod uart;




pub use crate::pins::uart::UartPin;


pub use self::config::UartConfig;
pub use self::frame::UartFrame;
pub use self::uart::*;

