//! Hardware locks of the RP2040.


//mod alloc;
mod default;
mod system;


//pub use self::alloc::*;
pub use self::default::*;
pub(crate) use self::system::*;