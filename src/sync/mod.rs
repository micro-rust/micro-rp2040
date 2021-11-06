//! Synchronization abstractions.


#![deny(warnings)]


mod fifo;
mod spinlock;
mod syslock;


pub use self::fifo::Mailbox;

pub use self::syslock::Syslock;

pub use self::spinlock::{
	Spinlock,

	Spinlock0,  Spinlock1,  Spinlock2,  Spinlock3,
	Spinlock4,  Spinlock5,  Spinlock6,  Spinlock7,
	Spinlock8,  Spinlock9,  Spinlock10, Spinlock11,
	Spinlock12, Spinlock13, Spinlock14, Spinlock15,
	Spinlock16, Spinlock17, Spinlock18, Spinlock19,
	Spinlock20, Spinlock21, Spinlock22, Spinlock23,
	Spinlock24, Spinlock25, Spinlock26, Spinlock27,
	Spinlock28, Spinlock29, Spinlock30,
};
