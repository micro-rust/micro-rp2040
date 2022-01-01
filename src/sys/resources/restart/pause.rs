//! Pause System for the Resources of the RP1040.


use crate::prelude::*;

use super::{ RUNNING, SystemResource };


pub(crate) struct PauseSystem;

impl PauseSystem {
    /// General function to indicate a System Resource is paused.
    #[inline(never)]
    pub(crate) unsafe fn pause(x: SystemResource) -> Result<(), SystemError> {
        // Get resource register and offset.
        let (r, o) = x.register();

        // Check if the resource is running.
        match RUNNING[r] & (1 << o) {
            0 => return Ok(()),
            _ => (),
        }


        // Get the masks.
        let m = x.children();

        // Check if the dependencies are already satisfied.
        if ((RUNNING[0] & m.0) != 0) || ((RUNNING[1] & m.1) != 0) {
            return Err( SystemError::UnresolvedDependencies );
        }

        // Acquire the resource.
        RUNNING[r] &= !(1 << o);

        Ok(())
    }
}
