//! Direct Memory Access (DMA) abstraction.

#![deny(warnings)]


pub(self) mod asynchronous;
mod channel;


pub mod stream;



pub use self::asynchronous::DMAHandle;
pub use self::channel::{ DMAChannel, DMAChannelTrait };
