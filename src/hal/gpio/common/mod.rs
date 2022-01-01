//! Common abstractions of the GPIO.


use crate::prelude::*;


mod base;
mod function;

pub use self::base::Gpio;
pub use self::function::AltFunction;


pub trait GPIOPin {
    const IO  : u32;
    const PAD : u32;

    fn intclear(&mut self);

    fn reset(&mut self);
}


/// Common function to clear the interrupts.
/// Common function to clear the previous configuration.
#[inline(never)]
pub(crate) fn intclear(r: usize, o: usize) {
    // Reference to the IRQ Raw registers.
    let intr = unsafe { &mut *(0x400140F0 as *mut [AtomicRegister<u32>; 40]) };

    // Clear triggerss
    intr[r +  0].clear(0xF << o);

    // Clear masks.
    intr[r +  4].clear(0xF << o);
    intr[r + 16].clear(0xF << o);
    intr[r + 28].clear(0xF << o);
}
