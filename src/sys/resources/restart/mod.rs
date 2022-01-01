//! Power State control system for System Resources.


use crate::prelude::*;

pub(super) mod pause;
pub(super) mod resume;

pub(self) use super::{ RUNNING, SystemResource };

pub(crate) use self::{ pause::PauseSystem, resume::ResumeSystem };


/// Common trait for all System Resources that can pause their functionality 
/// while waiting for system changes.
pub trait Restart {
    /// Function to pause the functionality of a System Resource.
    fn pause(&mut self) -> Result<(), SystemError>;

    /// Function to resume the functionality of a System Resource.
    fn resume(&mut self) -> Result<(), SystemError>;
}
