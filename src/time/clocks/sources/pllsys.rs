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

    static __HSLJ__ : u32;
    static __HSLP__ : u32;
    static __LSLJ__ : u32;
    static __LSLP__ : u32;
}

/// Maximum VCO frequency in MHz.
const VCOMAX : u32 = 1600;
/// Minimum VCO frequency in MHz.
const VCOMIN : u32 =  400;

/// Maximum feedback divider.
const FBDIVMAX : u32 = 320;
/// Minimum feedback divider.
const FBDIVMIN : u32 =  16;


/// Static reference to the System PLL Control peripheral.
type PLL = Peripheral<u32, AtomicRegister<u32>, 4, 0x40028000>;



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
        let mut PLL: PLL = Peripheral::get();

        // Reset the PLL.
        RESET.cycle(ResetId::PLLSYS);

        // Load reference divisor and feedback divisor.
        PLL[0].write(1);
        unsafe { PLL[2].write(__HSLJ__) };

        // Turn on PLL and VCO domains and wait for stabilization.
        PLL[1].set((1 << 5) | 1);
        while PLL[0].read() >> 31 == 0 { nop() }

        // Load post dividers.
        unsafe { PLL[3].write(__HSLJ__) };

        // Turn on post dividers.
        PLL[1].set(1 << 3);

        // Set the frequency.
        unsafe { CLOCKS.freqs[Clock::PllSys.index()] = ( XFREQ * (PLL[2].read() & 0xFFF) ) / 12; }
    }


    /// Load precomputed High Speed - Low Jitter PLL configuration.
    /// Produces a 133.33 MHz low jitter output.
    #[inline(always)]
    pub fn hslj(&self) -> Result<(), ()> {
        unsafe { self.load(__HSLJ__) }
    }

    /// Loads a precomputed High Speed - Low Power PLL configuration.
    /// Produces a 133.33 MHz output.
    #[inline(always)]
    pub fn hslp(&self) -> Result<(), ()> {
        unsafe { self.load(__HSLP__) }
    }

    /// Loads a precomputed High Speed - Low Jitter PLL configuration.
    /// Produces a 32.65 MHz output.
    #[inline(always)]
    pub fn lslj(&self) -> Result<(), ()> {
        unsafe { self.load(__LSLJ__) }
    }

    /// Precomputed High Speed - Low Power PLL configuration.
    /// Produces a 33.33 MHz output.
    #[inline(always)]
    pub fn lslp(&self) -> Result<(), ()> {
        unsafe { self.load(__LSLP__) }
    }


    /// Load shte given configuration data.
    fn load(&self, cfg: u32) -> Result<(), ()> {
        extern "C" {
            static XFREQ : u32;
        }

        let mut PLL: PLL = Peripheral::get();

        match Syslock::acquire() {
            Some(_) => match self.0.refs() {
                0 => {
                    // Reset the PLL.
                    RESET.cycle(ResetId::PLLSYS);

                    // Load reference divisor and feedback divisor.
                    PLL[0].write(1);
                    PLL[2].write(cfg);

                    // Turn on PLL and VCO domains and wait for stabilization.
                    PLL[1].set((1 << 5) | 1);
                    while PLL[0].read() >> 31 == 0 { nop() }

                    // Load post dividers.
                    PLL[3].write(cfg);

                    // Turn on post dividers.
                    PLL[1].set(1 << 3);

                    // Set the frequency.
                    unsafe { CLOCKS.freqs[Clock::PllSys.index()] = ( XFREQ * (cfg & 0xFFF) ) / 12; }

                    Ok(())
                },

                _ => Err(()),
            }

            _ => Err(()),
        }
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

    /// Module internal method to shut down the PLL.
    #[inline(always)]
    pub(super) fn off(&mut self) {
        let mut PLL: PLL = Peripheral::get();

        PLL[1].write(0);
    }
}
