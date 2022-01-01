//! Common trait and configuration for Clock Input pins.


use crate::prelude::*;


pub trait ClockIn<const N: usize>: GPIOPin {
    #[inline]
    fn config(&mut self) {
        // Clear interrupts.
        self.intclear();

        // Reference to the PAD register.
        let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Configure the pad.
        // Disable output, enable input, no Schmitt, slew fast-
        pad.write( (1 << 7) | (1 << 6) | 1 );


        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, disable output,
        // drive output from peripheral, select UART function.
        io[1].write( (0x2 << 12) | 8 );
    }
}

impl ClockIn<0> for Gpio<20> {}
impl ClockIn<1> for Gpio<22> {}
