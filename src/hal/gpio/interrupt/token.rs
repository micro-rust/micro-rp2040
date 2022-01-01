//! GPIO Interrupt Token.
//! Locks a GPIO Interrupt configuration but allows for future reconfiguration.


use crate::prelude::*;

use super::GPIOInterrupt;


pub struct GPIOInterruptToken<const N: u8>;


impl<const N: u8> GPIOInterruptToken<N> {
    /// Initializer.
    #[inline(always)]
    pub(super) fn create() -> Self {
        Self
    }

    /// Disables the IRQs and returns to configuration phase.
    pub fn reconfigure(&mut self) -> GPIOInterrupt<N> {
        core::mem::forget(self);

        GPIOInterrupt::create( Gpio )
    }
}


impl<const N: u8> Release for GPIOInterruptToken<N> {
    fn release(&mut self) {
        GPIOInterrupt::create( Gpio ).release();

        core::mem::forget(self);
    }
}

impl<const N: u8> Drop for GPIOInterruptToken<N> {
    fn drop(&mut self) {
        GPIOInterrupt::create( Gpio ).release();
    }
}
