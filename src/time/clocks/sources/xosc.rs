//! Crystal Oscillator wrapper.


use crate::raw::AtomicRegister;
use crate::sync::Syslock;
use crate::time::CLOCKS;
use crate::time::clocks::{ Clock, ClockInfo };

use micro::Peripheral;
use micro::Register;
use micro::asm::nop;



extern "C" {
    static XFREQ : u32;
    static XMULTIPLIER : u32;
}

/// Precomputed XOSC delay.
const DELAY : u32 = unsafe { (((XFREQ / 1000) + 128) / 256) * XMULTIPLIER };


/// Static reference to the XOSC Control peripheral.
static mut XOSC : Peripheral<u32, AtomicRegister<u32>, 3, 0x40024000> = Peripheral::get();



/// clock Info wrapper for the Crystal Oscillator.
#[repr(transparent)]
pub struct Xosc(ClockInfo);


impl Xosc {
    /// Static initializer.
    #[inline(always)]
    pub const fn empty() -> Xosc {
        Xosc(ClockInfo::empty())
    }

    /// Initializes the Crystal Oscillator.
    pub(crate) fn init(&mut self) {
        // Set startup delay.
        XOSC[3].write(DELAY);

        // Set enable status and frequency range.
        XOSC[0].write( (0xFAB << 12) | 0xAA0 );

        // Wait for stable XOSC.
        while (XOSC[1].read() >> 31) == 0 { nop() }
    }

    /// Returns the current frequency.
    #[inline(always)]
    pub fn freq(&self) -> u32 {
        unsafe { CLOCKS.freqs[Clock::Xosc.index()] }
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
                Some( unsafe { CLOCKS.freqs[Clock::Xosc.index()] } )
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
