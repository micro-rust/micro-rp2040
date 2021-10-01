//! USB PLL wrapper.


use crate::power::{ RESET, ResetId };
use crate::raw::AtomicRegister;
use crate::sync::Syslock;
use crate::time::CLOCKS;
use crate::time::clocks::{ Clock, ClockInfo };

use micro::Peripheral;
use micro::Register;
use micro::asm::nop;



extern "C" {
    static XFREQ : u32;
}

/// Maximum VCO frequency in MHz.
const VCOMAX : u32 = 1600;
/// Minimum VCO frequency in MHz.
const VCOMIN : u32 =  400;

/// Maximum feedback divider.
const FBDIVMAX : u32 = 320;
/// Minimum feedback divider.
const FBDIVMIN : u32 =  16;


/// Static reference to the XOSC Control peripheral.
static mut PLL : Peripheral<u32, AtomicRegister<u32>, 4, 0x40028000> = Peripheral::get();

/// Precomputed 48 MHz configuration.
static CONFIG : u32 = (6 << 16) | (5 << 12) | unsafe { 1_440_000 / (XFREQ / 1000) };



/// Clock Info wrapper for the System PLL.
#[repr(transparent)]
pub struct PllUsb(ClockInfo);


impl PllUsb {
    /// Static initializer.
    #[inline(always)]
    pub const fn empty() -> PllUsb {
        PllUsb(ClockInfo::empty())
    }

    /// Initializes the System PLL to 133 MHz low jitter.
    pub(crate) fn init(&mut self) {
        // Reset the PLL.
        RESET.cycle(ResetId::PLLUSB);

        // Load reference divisor and feedback divisor.
        PLL[0].write(1);
        PLL[2].write(CONFIG);

        // Turn on PLL and VCO domains and wait for stabilization.
        PLL[1].set((1 << 5) | 1);
        while PLL[0].read() >> 31 == 0 { nop() }

        // Load post dividers.
        PLL[3].write(CONFIG);

        // Turn on post dividers.
        PLL[1].set(1 << 3);
    }

    /// Returns the current frequency.
    #[inline(always)]
    pub fn freq(&self) -> u32 {
        unsafe { CLOCKS.freqs[Clock::PllUsb.index()] }
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
                Some( unsafe { CLOCKS.freqs[Clock::PllUsb.index()] } )
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
