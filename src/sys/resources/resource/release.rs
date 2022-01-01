//! Controls the release of resources.


use crate::prelude::*;

use super::{ AVAILABLE, RUNNING, SystemResource };



pub(crate) struct ReleaseSystem;


impl ReleaseSystem {
    /// General function to release resources.
    /// UNSAFETY: Assumes the System lock is already acquired.
    #[inline(never)]
    pub(crate) unsafe fn release(x: SystemResource) -> Result<(), SystemError> {
        // Get resource register and offset.
        let (r, o) = x.register();

        // Check if the resource is available.
        match AVAILABLE[r] & (1 << o) {
            0 => (),
            _ => return Err( SystemError::ResourceAlreadyReleased ),
        }

        // Get the masks.
        let m = x.children();

        // Check if the dependencies are already satisfied.
        if ((RUNNING[0] & m.0) != 0) || ((RUNNING[1] & m.1) != 0) {
            return Err( SystemError::UnresolvedDependencies );
        }

        // Acquire the resource.
        AVAILABLE[r] |= 1 << o;
        RUNNING[r] &= !(1 << o);

        Ok(())
    }

    /// General function to release resources.
    /// UNSAFETY: Assumes the System lock is already acquired.
    #[inline(never)]
    pub(crate) unsafe fn force(x: SystemResource) {
        // Get resource register and offset.
        let (r, o) = x.register();

        // Acquire the resource.
        AVAILABLE[r] |= 1 << o;
        RUNNING[r] &= !(1 << o);
    }
}
