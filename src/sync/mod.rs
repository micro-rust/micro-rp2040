//! Synchronization abstractions.


#![deny(warnings)]


mod fifo;
mod lock;


pub use self::fifo::Mailbox;

pub use self::lock::*;