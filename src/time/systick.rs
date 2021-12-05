//! System tick peripheral.
//! Simply specifies the Register type to allow for Atomic Hardware accesses.


use crate::prelude::*;

use micro::scb;


#[repr(transparent)]
pub struct Systick( scb::Systick<SIORegister<u32>> );


impl Systick {
    /// Sets the clock source as the external clock.
    #[inline(always)]
    pub fn external(&mut self) {
        self.0.external()
    }

    /// Sets the clock source as the processor's clock.
    #[inline(always)]
    pub fn processor(&mut self) {
        self.0.processor()
    }

    /// Enables / Disables the Systick interrupt.
    #[inline(always)]
    pub fn interrupt(&mut self, s: bool) {
        self.0.interrupt(s)
    }

    /// Enables the system timer.
    #[inline(always)]
    pub fn enable(&mut self) {
        self.0.enable()
    }

    /// Disables the system timer.
    #[inline(always)]
    pub fn disable(&mut self) {
        self.0.disable()
    }

    /// Reloads the system timer with this countdown value.
    #[inline(always)]
    pub fn reload(&mut self, rvr: u32) {
        self.0.reload(rvr)
    }

    /// Returns the current count.
    #[inline(always)]
    pub fn current() -> u32 {
        scb::Systick::<SIORegister<u32>>::current()
    }
}

impl SystemResource for Systick {
    fn acquire() -> Result<Self, SystemError> {
        match Resources::systick() {
            Some(_) => Ok(Self(scb::Systick::empty())),

            _ => Err( SystemError::PeripheralNotAvailable ),
        }
    }

    fn release(&mut self) {
        DropResources::systick();

        core::mem::forget(self);
    }
}

impl Drop for Systick {
    fn drop(&mut self) {
        DropResources::systick();
    }
}
