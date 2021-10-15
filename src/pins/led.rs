//! Led Pin abstraction.


use crate::raw::AtomicRegister;
use crate::raw::SIORegister;

use micro::Register;

use super::*;


/// UART Pin object. Can only be moved.
pub struct LedPin<const N: u32>;

impl<const N: u32> LedPin<N> {
    const FUNCSEL : u32 = 5;

    #[inline(always)]
    pub const fn from(_: Gpio<N>) -> Self {
        Self
    }

    /// Initializes the LED Pin.
    #[inline(always)]
    pub fn init(&self) {
        // SIO Output Enable Clear reference.
        // Enable output.
        let oeset: &'static mut SIORegister<u32> = unsafe { &mut *(0xD0000024 as *mut _) };
        oeset.write(1 << N);

        // SIO Output Clear reference.
        // Set output low.
        let oclr: &'static mut SIORegister<u32> = unsafe { &mut *(0xD0000018 as *mut _) };
        oclr.write(1 << N);

        // IO Bank.
        // Set Function as SIO.
        let io: &'static mut [AtomicRegister<u32>; 2] = unsafe { &mut *(Self::IO as *mut _) };
        io[1].write(Self::FUNCSEL);
    }

    /// Turns on the LED.
    #[inline(always)]
    pub fn on(&self) {
        let sio: &'static mut SIORegister<u32> = unsafe { &mut *(0xD0000014 as *mut _) };
        sio.write(1 << N);
    }

    /// Turns off the LED.
    #[inline(always)]
    pub fn off(&self) {
        let sio: &'static mut SIORegister<u32> = unsafe { &mut *(0xD0000018 as *mut _) };
        sio.write(1 << N);
    }
}



impl<const N : u32> PinTrait for LedPin<N> {
    const IO  : u32 = 0x40014000 + {0x08 * N};
    const PAD : u32 = 0x4001C000 + {0x04 * N} + 0x04;
}
