//! System PLL wrapper.


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

/// Precomputed High Speed - Low Jitter PLL configuration.
/// Produces a 133.33 MHz output.
static HSLJ : u32 = (4 << 16) | (3 << 12) | unsafe { 1_600_000 / (XFREQ / 1000) };
/// Precomputed High Speed - Low Power PLL configuration.
/// Produces a 133.33 MHz output.
static HSLP : u32 = (3 << 16) | (1 << 12) | unsafe {   400_000 / (XFREQ / 1000) };

/// Precomputed High Speed - Low Jitter PLL configuration.
/// Produces a 32.65 MHz output.
static LSLJ : u32 = (7 << 16) | (7 << 12) | unsafe { 1_600_000 / (XFREQ / 1000) };
/// Precomputed High Speed - Low Power PLL configuration.
/// Produces a 33.33 MHz output.
static LSLP : u32 = (4 << 16) | (3 << 12) | unsafe {   400_000 / (XFREQ / 1000) };



/// Clock Info wrapper for the System PLL.
#[repr(transparent)]
pub struct PllSystem(ClockInfo);


impl PllSystem {
    /// Static initializer.
    #[inline(always)]
    pub const fn empty() -> PllSystem {
        PllSystem(ClockInfo::empty())
    }

    /// Initializes the System PLL to 133 MHz low jitter.
    pub(crate) fn init(&mut self) {
        // Reset the PLL.
        RESET.cycle(ResetId::PLLSYS);

        // Load reference divisor and feedback divisor.
        PLL[0].write(1);
        PLL[2].write(HSLJ);

        // Turn on PLL and VCO domains and wait for stabilization.
        PLL[1].set((1 << 5) | 1);
        while PLL[0].read() >> 31 == 0 { nop() }

        // Load post dividers.
        PLL[3].write(HSLJ);

        // Turn on post dividers.
        PLL[1].set(1 << 3);
    }

    /// Returns the current frequency.
    #[inline(always)]
    pub fn freq(&self) -> u32 {
        unsafe { CLOCKS.freqs[Clock::PllSys.index()] }
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
                Some( unsafe { CLOCKS.freqs[Clock::PllSys.index()] } )
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
