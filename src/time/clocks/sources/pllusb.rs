//! USB PLL wrapper.


use crate::features::__XFREQ__;
use crate::sys::power::{ RESET, ResetId };
use crate::raw::AtomicRegister;
use crate::sync::Syslock;
use crate::time::CLOCKS;
use crate::time::clocks::{ Clock, ClockInfo };

use micro::Peripheral;
use micro::Register;
use micro::asm::nop;



/// Maximum VCO frequency in MHz.
const VCOMAX : u32 = 1600;
/// Minimum VCO frequency in MHz.
const VCOMIN : u32 =  400;

/// Maximum feedback divider.
const FBDIVMAX : u32 = 320;
/// Minimum feedback divider.
const FBDIVMIN : u32 =  16;


/// Static reference to the USB PLL Control peripheral.
type PLL = Peripheral<u32, AtomicRegister<u32>, 4, 0x4002C000>;


static __USBCONFIG__: u32 = (6 << 16) | (5 << 12) | ( 1440000 / (__XFREQ__ / 1000));


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
        let mut PLL: PLL = Peripheral::get();

        // Reset the PLL.
        RESET.cycle(ResetId::PLLUSB);

        // Load reference divisor and feedback divisor.
        PLL[0].write(1);
        PLL[2].write(__USBCONFIG__);

        // Turn on PLL and VCO domains and wait for stabilization.
        PLL[1].clear((1 << 5) | 1);
        while PLL[0].read() >> 31 == 0 { nop() }

        // Load post dividers.
        PLL[3].write(__USBCONFIG__);

        // Turn on post dividers.
        PLL[1].clear(1 << 3);

        // Set the frequency.
        unsafe { CLOCKS.freqs[Clock::PllSys.index()] = ( __XFREQ__ * (PLL[2].read() & 0xFFF) ) / 30; }
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

    /// Module internal method to shut down the PLL.
    #[inline(always)]
    pub(super) fn off(&mut self) {
        let mut PLL: PLL = Peripheral::get();
        PLL[1].write(0xFF);
    }
}
