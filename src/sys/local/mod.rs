//! Local System control.
//! Access and control to system configurations and hardware that are core
//! exclusive. These configurations are NOT protected by a mutex and cannot
//! fail, although they can still be unsafe.


/// Hardware mutex module.
pub mod lock;