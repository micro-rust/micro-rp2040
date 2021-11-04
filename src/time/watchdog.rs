//! Watchdog timer of the RP2040.

use crate::raw::AtomicRegister;

use micro::Register;


pub struct Watchdog;

impl Watchdog {
    /// Sets the countdown value (24 bits) of the watchdog.
    /// WARNING: Due to a hardware bug, the watchdog counts twice as fast.
    #[inline(always)]
    pub fn countdown(&mut self, val: u32) {
        let WATCHDOG = unsafe { &mut *(0x40058000 as *mut [AtomicRegister<u32>; 3]) };

        WATCHDOG[1].write(val);
    }

    /// Enables the watchdog timer.
    #[inline(always)]
    pub fn enable(&mut self) {
        let WATCHDOG = unsafe { &mut *(0x40058000 as *mut [AtomicRegister<u32>; 3]) };

        WATCHDOG[0].set(1 << 30);
    }

    /// Forces a trigger of the watchdog timer.
    #[inline(always)]
    pub fn trigger(&mut self) {
        let WATCHDOG = unsafe { &mut *(0x40058000 as *mut [AtomicRegister<u32>; 3]) };

        WATCHDOG[0].set(1 << 31);
    }

    /// Allows access to 32 bytes of persistent memory.
    /// The bus access to these bytes is done in words (32 bits).
    pub fn persistence<'a>() -> &'a mut [u32; 8] {
        unsafe { &mut *(0x4005800C as *mut _) }
    }

    /// Sets a reboot function.
    /// This function will be called if the RP2040 is reset by the watchdog.
    pub fn reboot(entry: u32, sp: u32) {
        let SCRATCH = unsafe { &mut *(0x4005800C as *mut [AtomicRegister<u32>; 8]) };

        SCRATCH[4].write( 0xB007C0D3 );
        SCRATCH[5].write( (!0xB007C0D3) ^ entry );
        SCRATCH[6].write( sp );
        SCRATCH[7].write( entry );
    }
}
