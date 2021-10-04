//! Reference Clock wrapper.


use crate::raw::AtomicRegister;
use crate::sync::Syslock;
use crate::time::CLOCKS;
use crate::time::clocks::{ Clock, ClockInfo };

use micro::Peripheral;
use micro::Register;
use micro::asm::nop;


/// Static reference to the Reference Clock Control peripheral.
static mut CLOCK : Peripheral<u32, AtomicRegister<u32>, 3, 0x40008030> = Peripheral::get();


/// Clock Info wrapper for the Reference Clock.
#[repr(transparent)]
pub struct Reference(ClockInfo);


impl Reference {
    /// Static initializer.
    #[inline(always)]
    pub const fn empty() -> Reference {
        Reference(ClockInfo::empty())
    }

    /// Initializes the Reference Clock to the XOSC.
    pub(crate) fn init(&mut self) {
        extern "C" { static XFREQ : u32; }

        // Clear divider to set as 2^16.
        CLOCK[1].write(0);

        // Switch to the ROSC.
        CLOCK[0].write(0);

        // Wait until multiplexer has fininshed change.
        while CLOCK[2].read() == 0 { nop() }

        // Write the divider.
        CLOCK[1].write(1 << 8);

        // Setup information.
        unsafe { CLOCKS.freqs[Clock::Reference.index()] = XFREQ };
        self.0.info = (Clock::Rosc, Clock::PllUsb);
    }

    /// Switch to the auxiliary clock.
    pub fn aux(&mut self) -> Result<(), ()> {
        match Syslock::acquire() {
            Some(_) => match self.0.refs {
                0 => {
                    CLOCK[0].clear(0x3);
                    CLOCK[0].set(0x1);
                    while CLOCK[2].read() == 0 { nop() }

                    self.0.info = (Clock::Auxiliary, self.0.info.1);
                    unsafe {
                        CLOCKS.freqs[Clock::Reference.index()] = CLOCKS.freqs[self.0.info.1.index()];
                    }

                    Ok(())
                },
                _ => Err(()),
            },

            _ => Err(()),
        }
    }

    /// Switch to the Ring Oscillator.
    pub fn rosc(&mut self) -> Result<(), ()> {
        match Syslock::acquire() {
            Some(_) => match self.0.refs {
                0 => {
                    CLOCK[0].clear(0x3);
                    while CLOCK[2].read() == 0 { nop() }

                    self.0.info = (Clock::Rosc, self.0.info.1);
                    unsafe {
                        CLOCKS.freqs[Clock::Reference.index()] = CLOCKS.freqs[Clock::Rosc.index()];
                    }

                    Ok(())
                },
                _ => Err(()),
            },

            _ => Err(()),
        }
    }

    /// Switch to the Crystal Oscillator.
    pub fn xosc(&mut self) -> Result<(), ()> {
        match Syslock::acquire() {
            Some(_) => match self.0.refs {
                0 => {
                    CLOCK[0].clear(0x3);
                    CLOCK[0].set(0x2);
                    while CLOCK[2].read() == 0 { nop() }

                    self.0.info = (Clock::Rosc, self.0.info.1);
                    unsafe {
                        CLOCKS.freqs[Clock::Reference.index()] = CLOCKS.freqs[Clock::Xosc.index()];
                    }

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
        match Syslock::acquire() {
            Some(_) => match self.0.refs {

                // If it's not referenced, switch without fear.
                0 => self.__secondary__(clock),

                // If it's referenced, check if the secondary is used.
                _ => match CLOCK[0].read() & 0x3 {
                    0x1 => Err(()),
                    _ => self.__secondary__(clock),
                },
            },

            _ => Err(()),
        }
    }

    /// Inner method to set the secondary clock.
    fn __secondary__(&mut self, clock: Clock) -> Result<(), ()> {
        match clock {
            Clock::PllUsb => {
                CLOCK[0].clear(0x3 << 5);
                self.0.info.1 = clock;
            },
            Clock::GPInput0 => {
                CLOCK[0].clear(0x3 << 5);
                CLOCK[0].set(0x1 << 5);
                self.0.info.1 = clock;
            },
            Clock::GPInput1 => {
                CLOCK[0].clear(0x3 << 5);
                CLOCK[0].set(0x2 << 5);
                self.0.info.1 = clock;
            },
        }

        Ok(())
    }

    /// Returns the current clock frequency.
    #[inline(always)]
    pub fn freq(&self) -> u32 {
        unsafe { CLOCKS.freqs[Clock::Reference.index()] }
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

                Some( unsafe { CLOCKS.freqs[Clock::Reference.index()] } )
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
