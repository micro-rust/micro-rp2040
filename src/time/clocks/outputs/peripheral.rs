//! Reference Clock wrapper.


use crate::raw::AtomicRegister;
use crate::sync::Syslock;
use crate::time::CLOCKS;
use crate::time::clocks::{ Clock, ClockInfo };

use micro::Peripheral;
use micro::Register;


/// Static reference to the Peripheral Clock Control peripheral.
type CLOCK = Peripheral<u32, AtomicRegister<u32>, 1, 0x40008048>;


/// Clock Info wrapper for the Peripheral Clock.
#[repr(transparent)]
pub struct PeripheralClock(ClockInfo);


impl PeripheralClock {
    /// Static initializer.
    #[inline(always)]
    pub const fn empty() -> PeripheralClock {
        PeripheralClock(ClockInfo::empty())
    }

    /// Initializes the Peripheral Clock to the XOSC.
    pub(crate) fn init(&mut self) {
        let mut CLOCK: CLOCK = Peripheral::get();

        // Kill the clock.
        CLOCK[0].write(0);

        // Enable for system input.
        CLOCK[0].set(1 << 11);

        // Configure information.
        self.0.info.0 = Clock::System;

        unsafe { CLOCKS.freqs[Clock::Peripheral.index()] = CLOCKS.freqs[Clock::System.index()]; }
    }

    /// Returns the current clock frequency.
    #[inline(always)]
    pub fn freq(&self) -> u32 {
        unsafe { CLOCKS.freqs[Clock::Peripheral.index()] }
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

                Some( unsafe { CLOCKS.freqs[Clock::Peripheral.index()] } )
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
