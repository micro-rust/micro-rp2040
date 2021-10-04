//! Clock information.
//! Contains all the information necessary for clock configuration.


use super::Clock;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClockInfo {
    /// Reference counter for this clock.
    pub(crate) refs: u32,

    /// Clock source.
    pub(crate) info: (Clock, Clock),
}


impl ClockInfo {
    /// Static initializer.
    #[inline(always)]
    pub const fn empty() -> ClockInfo {
        ClockInfo {
            refs: 0u32,
            info: (Clock::None, Clock::None),
        }
    }

    /// Crate internal method to freeze without a System lock.
    #[inline(always)]
    pub(crate) fn __freeze__(&mut self) {
        self.refs += 1;
    }

    /// Returns the current number of references of the Clock.
    #[inline(always)]
    pub fn refs(&self) -> u32 {
        self.refs
    }
}