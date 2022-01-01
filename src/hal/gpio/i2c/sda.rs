//! Common trait and configuration for I2C SDA pins.


use crate::prelude::*;


pub trait I2CSda<const N: usize>: GPIOPin {
    #[inline]
    fn cpol1(&mut self) {
        // Clear interrupts.
        self.intclear();

        // Reference to the PAD register.
        let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Configure the pad.
        // Enable input, Pull Up, no Schmitt, slew fast-
        pad.write( (1 << 6) | (1 << 3) | 1 );


        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, disable output,
        // drive output from peripheral, select UART function.
        io[1].write( (0x3 << 12) | 2 );
    }

    #[inline]
    fn cpol0(&mut self) {
        // Clear interrupts.
        self.intclear();

        // Reference to the PAD register.
        let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Configure the pad.
        // Enable input, Pull Up, no Schmitt, slew fast-
        pad.write( (1 << 6) | (1 << 2) | 1 );


        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, disable output,
        // drive output from peripheral, select UART function.
        io[1].write( (0x3 << 12) | 2 );
    }
}


impl I2CSda<0> for Gpio<00> {}
impl I2CSda<1> for Gpio<02> {}
impl I2CSda<0> for Gpio<04> {}
impl I2CSda<1> for Gpio<06> {}
impl I2CSda<0> for Gpio<08> {}
impl I2CSda<1> for Gpio<10> {}
impl I2CSda<0> for Gpio<12> {}
impl I2CSda<1> for Gpio<14> {}
impl I2CSda<0> for Gpio<16> {}
impl I2CSda<1> for Gpio<18> {}
impl I2CSda<0> for Gpio<20> {}
impl I2CSda<1> for Gpio<22> {}
impl I2CSda<0> for Gpio<24> {}
impl I2CSda<1> for Gpio<26> {}
impl I2CSda<0> for Gpio<28> {}
