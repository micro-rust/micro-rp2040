//! Common trait and configuration for UART RX pins.


use crate::prelude::*;


pub trait UartRx<const N: usize>: GPIOPin {
    #[inline]
    fn config(&mut self) {
        // Clear interrupts.
        self.intclear();

        // Reference to the PAD register.
        let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Configure the pad.
        // Disable output, enable input, Pull Up, no Schmitt, slew fast-
        pad.write( (1 << 7) | (1 << 6) | (1 << 3) | 1 );


        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, disable output,
        // drive output from peripheral, select UART function.
        io[1].write( (0x2 << 12) | 2 );
    }
}

impl UartRx<0> for Gpio<01> {}
impl UartRx<1> for Gpio<05> {}
impl UartRx<1> for Gpio<09> {}
impl UartRx<0> for Gpio<13> {}
impl UartRx<0> for Gpio<17> {}
impl UartRx<1> for Gpio<21> {}
impl UartRx<1> for Gpio<25> {}
impl UartRx<0> for Gpio<29> {}
