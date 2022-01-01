//! Controls the acquisition of resources.


use crate::prelude::*;

use super::{ AVAILABLE, RUNNING, SystemResource };


pub(crate) struct AcquireSystem;


impl AcquireSystem {
    /// General function to acquire resources.
    /// UNSAFETY: Assumes the System lock is already acquired.
    #[inline(never)]
    pub(crate) unsafe fn acquire(x: SystemResource) -> Result<(), SystemError> {
        // Get resource register and offset.
        let (r, o) = x.register();

        // Check if the resource is available.
        match AVAILABLE[r] & (1 << o) {
            0 => return Err( SystemError::ResourceCurrentlyUsed ),
            _ => (),
        }

        // Get the masks.
        let m = x.parents();

        // Check if the dependencies are already satisfied.
        if ((RUNNING[0] & m.0) != m.0) || ((RUNNING[1] & m.1) != m.1) {
            return Err( SystemError::UnresolvedDependencies );
        }

        // Acquire the resource.
        AVAILABLE[r] &= !(1 << o);
        RUNNING[r] |= 1 << o;

        Ok(())
    }
}
