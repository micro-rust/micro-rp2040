//! Common trait and configuration for Clock Output pins.


use crate::prelude::*;


pub trait ClockOut<const N: usize>: GPIOPin {
    #[inline]
    fn config(&mut self) {
        // Clear interrupts.
        self.intclear();

        // Reference to the PAD register.
        let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Configure the pad.
        // Disable output, enable input, no Schmitt, slew fast-
        pad.write( 1 );


        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, disable output,
        // drive output from peripheral, select UART function.
        io[1].write( 8 );
    }
}

impl ClockOut<0> for Gpio<21> {}
impl ClockOut<1> for Gpio<23> {}
impl ClockOut<2> for Gpio<24> {}
impl ClockOut<3> for Gpio<25> {}
