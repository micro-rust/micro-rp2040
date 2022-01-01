//! Common trait and configuration for UART TX pins.


use crate::prelude::*;


pub trait UartTx<const N: usize>: GPIOPin {
    #[inline]
    fn config(&mut self) {
        // Clear interrupts.
        self.intclear();

        // Reference to the PAD register.
        let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Configure the pad.
        // Pull Up, slew fast.
        pad.write( (1 << 3) | 1 );


        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, disable output,
        // drive output from peripheral, select UART function.
        io[1].write( (0x3 << 12) | 2 );
    }
}

impl UartTx<0> for Gpio<00> {}
impl UartTx<1> for Gpio<04> {}
impl UartTx<1> for Gpio<08> {}
impl UartTx<0> for Gpio<12> {}
impl UartTx<0> for Gpio<16> {}
impl UartTx<1> for Gpio<20> {}
impl UartTx<1> for Gpio<24> {}
impl UartTx<0> for Gpio<28> {}
