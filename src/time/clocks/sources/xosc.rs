//! Crystal Oscillator wrapper.


use crate::features::{ __XFREQ__, __DELAY__ };
use crate::raw::AtomicRegister;
use crate::sync::Syslock;
use crate::time::CLOCKS;
use crate::time::clocks::{ Clock, ClockInfo };

use micro::Peripheral;
use micro::Register;
use micro::asm::nop;



/// Type of the XOSC Control peripheral.
type XOSC = Peripheral<u32, AtomicRegister<u32>, 4, 0x40024000>;



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
    #[inline(never)]
    pub(crate) fn init(&mut self) {
        let mut xosc: XOSC = Peripheral::get();

        // Set input frequency.
        xosc[0].write(0xAA0);

        // Set startup delay.
        xosc[3].write(__DELAY__);

        // Set enable status and frequency range.
        xosc[0].write( (0xFAB << 12) | 0xAA0 );

        // Wait for stable XOSC.
        while (xosc[1].read() >> 31) == 0 { nop() }

        // Clear possible initial bad write.
        xosc[1].set(1 << 24);

        // Set the frequencies.
        unsafe { CLOCKS.freqs[Clock::Xosc.index()] = __XFREQ__; }
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
