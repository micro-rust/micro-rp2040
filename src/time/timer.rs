//! Module for the 64 bit timer and timers based on it.

use crate::raw::AtomicRegister;

use micro::Register;


/// Microsecond timer with microseconds resolution.
/// Thousands of years before overflow.
pub struct USTimer;

impl USTimer {
    /// Reads the pair of values of the timer.
    /// Returns `(high, low)`.
    /// This function is always safe.
    #[inline(always)]
    pub fn read() -> (u32, u32) {
        let TIMER = unsafe { &mut *(0x40054000 as *mut [AtomicRegister<u32>; 4]) };

        ( TIMER[3].read(), TIMER[2].read() )
    }
}
