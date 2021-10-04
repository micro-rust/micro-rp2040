//! Ring Oscillator wrapper.


use crate::raw::AtomicRegister;
use crate::sync::Syslock;
use crate::time::CLOCKS;
use crate::time::clocks::{ Clock, ClockInfo };

use micro::Peripheral;


/// Static reference to the ROSC Control peripheral.
static mut ROSC : Peripheral<u32, AtomicRegister<u32>, 8, 0x40060000> = Peripheral::get();



/// clock Info wrapper for the Ring Oscillator.
#[repr(transparent)]
pub struct Rosc(ClockInfo);


impl Rosc {
    /// Static initializer.
    #[inline(always)]
    pub const fn empty() -> Rosc {
        Rosc(ClockInfo::empty())
    }

    /// Initializes the Ring Oscillator.
    pub(crate) fn init(&mut self) {

    }

    /// Returns the current frequency.
    #[inline(always)]
    pub fn freq(&self) -> u32 {
        unsafe { CLOCKS.freqs[Clock::Rosc.index()] }
    }

    /// Returns the current reference counter.
    #[inline(always)]
    pub fn refs(&self) -> u32 {
        self.0.refs()
    }

    /// Freezes the clock.
    pub fn freeze(&mut self) -> Option<u32> {
        match Syslock::acquire() {
            Some(_) => {
                self.0.__freeze__();
                Some( unsafe { CLOCKS.freqs[Clock::Rosc.index()] } )
            },
            _ => None
        }
    }

    /// Crate internal method to freeze without a System lock.
    #[inline(always)]
    pub(crate) fn __freeze__(&mut self) {
        self.0.__freeze__()
    }
}
