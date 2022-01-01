//! Power State control system for System Resources.


use crate::prelude::*;

mod acquire;
mod release;

pub(self) use super::{ AVAILABLE, RUNNING, SystemResource };

pub(crate) use self::{ acquire::AcquireSystem, release::ReleaseSystem };


/// Common trait for all System Resources that can be acquired.
pub trait Acquire: Sized + Release {
    /// Function to acquire the resource.
    /// This function also powers on/enables the resource.
    fn acquire() -> Result<Self, SystemError>;
}



/// Common trait for all System Resources that need to be released orderly.
pub trait Release: Sized + Drop {
    /// Function to release the resource.
    /// This function also powers off / disables the resource.
    fn release(&mut self) -> Result<(), SystemError>;
}
