//! ADC Configuration for GPIO.


use crate::prelude::*;


pub trait ADCPin: GPIOPin {
    #[inline]
    fn config(&mut self) {
        // Clear interrupts.
        self.intclear();

        // Reference to the PAD register.
        let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Configure the pad.
        // Disable output.
        pad.write(1 << 7);


        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        io[1].write( (0x2 << 12) | 0x1F );
    }
}

impl ADCPin for Gpio<26> {}
impl ADCPin for Gpio<27> {}
impl ADCPin for Gpio<28> {}
impl ADCPin for Gpio<29> {}
