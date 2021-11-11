//! Initialization code for Core 1.


pub(super) fn initialize() {
    // Initialize interrupts.
    crate::sys::ints::InterruptSystem::init();
}
