//! USB Clock wrapper.


use crate::raw::AtomicRegister;
use crate::sync::Syslock;
use crate::sys::CLOCKS;
use crate::sys::clocks::{ Clock, ClockInfo };

use micro::Peripheral;
use micro::Register;


/// Static reference to the USB Clock Control peripheral.
type CLOCK = Peripheral<u32, AtomicRegister<u32>, 2, 0x40008054>;


/// Clock Info wrapper for the USB Clock.
#[repr(transparent)]
pub struct UsbClock(ClockInfo);


impl UsbClock {
    /// Static initializer.
    #[inline(always)]
    pub const fn empty() -> UsbClock {
        UsbClock(ClockInfo::empty())
    }

    /// Initializes the USB Clock to the USB PLL Output.
    pub(crate) fn init(&mut self) {
        let mut CLOCK: CLOCK = Peripheral::get();

        // Kill the clock.
        CLOCK[0].write(0);

        // Enable for system input.
        CLOCK[0].set(1 << 11);

        // Configure information.
        self.0.info.0 = Clock::PllUsb;

        unsafe { CLOCKS.freqs[Clock::Usb.index()] = CLOCKS.freqs[Clock::PllUsb.index()]; }
    }

    /// Returns the current clock frequency.
    #[inline(always)]
    pub fn freq(&self) -> u32 {
        unsafe { CLOCKS.freqs[Clock::Usb.index()] }
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

                Some( unsafe { CLOCKS.freqs[Clock::Usb.index()] } )
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