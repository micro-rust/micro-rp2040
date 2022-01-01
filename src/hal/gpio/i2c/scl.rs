//! Common trait and configuration for I2C SCL pins.


use crate::prelude::*;


pub trait I2CScl<const N: usize>: GPIOPin {
    #[inline]
    fn config(&mut self) {
        // Clear interrupts.
        self.intclear();

        // Reference to the PAD register.
        let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Configure the pad.
        // Enable output, Pull Up, slew fast.
        pad.write( (1 << 7) | (1 << 3) | 1 );


        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        io[1].write( (0x3 << 12) | 2 );
    }
}

impl I2CScl<0> for Gpio<01> {}
impl I2CScl<1> for Gpio<03> {}
impl I2CScl<0> for Gpio<05> {}
impl I2CScl<1> for Gpio<07> {}
impl I2CScl<0> for Gpio<09> {}
impl I2CScl<1> for Gpio<11> {}
impl I2CScl<0> for Gpio<13> {}
impl I2CScl<1> for Gpio<15> {}
impl I2CScl<0> for Gpio<17> {}
impl I2CScl<1> for Gpio<19> {}
impl I2CScl<0> for Gpio<21> {}
impl I2CScl<1> for Gpio<23> {}
impl I2CScl<0> for Gpio<25> {}
impl I2CScl<1> for Gpio<27> {}
impl I2CScl<0> for Gpio<29> {}
