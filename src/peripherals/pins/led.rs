//! Led Pin abstraction.


use crate::prelude::*;

use super::*;


const FUNCSEL: u32 = 5;


/// LED Pin trait.
pub trait LedPin<const N: usize>: PinTrait {
    #[inline]
    fn init(&self) {
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
        io[1].write(FUNCSEL);
    }


    /// Turns on the LED.
    #[inline(always)]
    fn on(&self) {
        let sio: &'static mut SIORegister<u32> = unsafe { &mut *(0xD0000014 as *mut _) };
        sio.write(1 << N);
    }

    /// Turns off the LED.
    #[inline(always)]
    fn off(&self) {
        let sio: &'static mut SIORegister<u32> = unsafe { &mut *(0xD0000018 as *mut _) };
        sio.write(1 << N);
    }
}


#[cfg(feature = "raspberry-pico")]
impl LedPin<25> for Gpio<25> {}