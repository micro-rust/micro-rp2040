//! System Clock wrapper.


use crate::raw::AtomicRegister;
use crate::sync::Syslock;
use crate::time::CLOCKS;
use crate::time::clocks::{ Clock, ClockInfo };

use micro::Peripheral;
use micro::Register;
use micro::asm::nop;


/// Static reference to the System Clock Control peripheral.
type CLOCK = Peripheral<u32, AtomicRegister<u32>, 3, 0x4000803C>;


/// Clock Info wrapper for the System Clock.
#[repr(transparent)]
pub struct SystemClock(ClockInfo);


impl SystemClock {
    /// Static initializer.
    #[inline(always)]
    pub const fn empty() -> SystemClock {
        SystemClock(ClockInfo::empty())
    }

    /// Initializes the System Clock to the XOSC.
    #[inline(never)]
    pub(crate) fn init(&mut self) {
        let mut CLOCK: CLOCK = Peripheral::get();

        // Switch to the Reference Clock.
        CLOCK[0].write(0);

        // Write the divider.
        CLOCK[1].write(1 << 8);

        // Switch to the System Pll.
        CLOCK[0].set(1);

        // Wait until the clock is stable.
        while CLOCK[2].read() == 0 { nop() }

        // Setup information.
        unsafe { CLOCKS.freqs[Clock::System.index()] = CLOCKS.freqs[Clock::PllSys.index()]; }
        self.0.info = (Clock::Auxiliary, Clock::PllSys);
    }

    /// Switch to the auxiliary clock.
    pub fn aux(&mut self) -> Result<(), ()> {
        let mut CLOCK: CLOCK = Peripheral::get();

        match Syslock::acquire() {
            Some(_) => match self.0.refs {
                0 => {
                    CLOCK[0].set(0x1);
                    while CLOCK[2].read() == 0 { nop() }

                    self.0.info = (Clock::Auxiliary, self.0.info.1);

                    Ok(())
                },
                _ => Err(()),
            },

            _ => Err(()),
        }
    }

    /// Switch to the Reference clock.
    pub fn reference(&mut self) -> Result<(), ()> {
        let mut CLOCK: CLOCK = Peripheral::get();

        match Syslock::acquire() {
            Some(_) => match self.0.refs {
                0 => {
                    CLOCK[0].clear(0x1);
                    while CLOCK[2].read() == 0 { nop() }

                    self.0.info = (Clock::Reference, self.0.info.1);

                    Ok(())
                },
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    /// Select the secondary clock.
    /// This method will fail if the secondary Reference Clock is in use.
    pub fn secondary(&mut self, clock: Clock) -> Result<(), ()> {
        let CLOCK: CLOCK = Peripheral::get();

        match Syslock::acquire() {
            Some(_) => match self.0.refs {

                // If it's not referenced, switch without fear.
                0 => self.__secondary__(clock),

                // If it's referenced, check if the secondary is used.
                _ => match CLOCK[0].read() & 0x1 {
                    0x1 => Err(()),
                    _ => self.__secondary__(clock),
                },
            },

            _ => Err(()),
        }
    }

    /// Inner method to set the secondary clock.
    fn __secondary__(&mut self, clock: Clock) -> Result<(), ()> {
        let mut CLOCK: CLOCK = Peripheral::get();

        match clock {
            Clock::PllSys => {
                CLOCK[0].clear(0x7 << 5);
                self.0.info.1 = clock;
            },
            Clock::PllUsb => {
                CLOCK[0].clear(0x7 << 5);
                CLOCK[0].set(0x1 << 5);
                self.0.info.1 = clock;
            },
            Clock::Rosc => {
                CLOCK[0].clear(0x7 << 5);
                CLOCK[0].set(0x2 << 5);
                self.0.info.1 = clock;
            },
            Clock::Xosc => {
                CLOCK[0].clear(0x7 << 5);
                CLOCK[0].set(0x3 << 5);
                self.0.info.1 = clock;
            },
            Clock::GPInput0 => {
                CLOCK[0].clear(0x7 << 5);
                CLOCK[0].set(0x4 << 5);
                self.0.info.1 = clock;
            },
            Clock::GPInput1 => {
                CLOCK[0].clear(0x7 << 5);
                CLOCK[0].set(0x5 << 5);
                self.0.info.1 = clock;
            },

            _ => return Err(()),
        }

        Ok(())
    }

    /// Returns the current frequency.
    #[inline(always)]
    pub fn freq(&self) -> u32 {
        unsafe { CLOCKS.freqs[Clock::System.index()] }
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

                Some( unsafe { CLOCKS.freqs[Clock::System.index()] } )
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
