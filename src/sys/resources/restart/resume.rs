//! Resume System for the Resources of the RP1040.


use crate::prelude::*;

use super::{ RUNNING, SystemResource };


pub(crate) struct ResumeSystem;

impl ResumeSystem {

    /// General function to resume resources.
    /// UNSAFETY: Assumes the System lock is already acquired.
    #[inline(never)]
    pub(crate) unsafe fn resume(x: SystemResource) -> Result<(), SystemError> {
        // Get resource register and offset.
        let (r, o) = x.register();

        // Check if the resource is running.
        match RUNNING[r] & (1 << o) {
            0 => (),
            _ => return Ok(()),
        }

        // Get the masks.
        let m = x.parents();

        // Check if the dependencies are already satisfied.
        if ((RUNNING[0] & m.0) != m.0) || ((RUNNING[1] & m.1) != m.1) {
            return Err( SystemError::UnresolvedDependencies );
        }

        // Acquire the resource.
        RUNNING[r] |= 1 << o;

        Ok(())
    }
}
