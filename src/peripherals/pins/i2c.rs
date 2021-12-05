//! I2C Pin abstractions.


use crate::prelude::*;

use super::*;



/// I2C Function selector.
const FUNCSEL : u32 = 3;



/// Common trait for UART pins.
pub trait I2CPin<const N: usize>: PinTrait {}


impl I2CPin<0> for Gpio<00> {}
impl I2CPin<0> for Gpio<01> {}

impl I2CPin<1> for Gpio<02> {}
impl I2CPin<1> for Gpio<03> {}

impl I2CPin<0> for Gpio<04> {}
impl I2CPin<0> for Gpio<05> {}

impl I2CPin<1> for Gpio<06> {}
impl I2CPin<1> for Gpio<07> {}

impl I2CPin<0> for Gpio<08> {}
impl I2CPin<0> for Gpio<09> {}

impl I2CPin<1> for Gpio<10> {}
impl I2CPin<1> for Gpio<11> {}

impl I2CPin<0> for Gpio<12> {}
impl I2CPin<0> for Gpio<13> {}

impl I2CPin<1> for Gpio<14> {}
impl I2CPin<1> for Gpio<15> {}

impl I2CPin<0> for Gpio<16> {}
impl I2CPin<0> for Gpio<17> {}

impl I2CPin<1> for Gpio<18> {}
impl I2CPin<1> for Gpio<19> {}

impl I2CPin<0> for Gpio<20> {}
impl I2CPin<0> for Gpio<21> {}

impl I2CPin<1> for Gpio<22> {}
impl I2CPin<1> for Gpio<23> {}

impl I2CPin<0> for Gpio<24> {}
impl I2CPin<0> for Gpio<25> {}

impl I2CPin<1> for Gpio<26> {}
impl I2CPin<1> for Gpio<27> {}

impl I2CPin<0> for Gpio<28> {}
impl I2CPin<0> for Gpio<29> {}




/// Common trait for I2C SDA pins.
pub trait I2CSdaPin<const N: usize>: I2CPin<N> {
    #[inline(always)]
    fn config(&self) {
        // Reference to the PAD register.
        let _pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, drive output enable from peripheral,
        // drive output from peripheral, select UART function.
        io[1].write(FUNCSEL & 0x1F);

        // Configure the pad.
        // Enable output, disable input, drive to 4 mA, Pull Up, Schmitt, Slew limited.
        //pad.write((1 << 3) | (1 << 1));
    }
}


impl I2CSdaPin<0> for Gpio<00> {}
impl I2CSdaPin<1> for Gpio<02> {}
impl I2CSdaPin<0> for Gpio<04> {}
impl I2CSdaPin<1> for Gpio<06> {}
impl I2CSdaPin<0> for Gpio<08> {}
impl I2CSdaPin<1> for Gpio<10> {}
impl I2CSdaPin<0> for Gpio<12> {}
impl I2CSdaPin<1> for Gpio<14> {}
impl I2CSdaPin<0> for Gpio<16> {}
impl I2CSdaPin<1> for Gpio<18> {}
impl I2CSdaPin<0> for Gpio<20> {}
impl I2CSdaPin<1> for Gpio<22> {}
impl I2CSdaPin<0> for Gpio<24> {}
impl I2CSdaPin<1> for Gpio<26> {}
impl I2CSdaPin<0> for Gpio<28> {}



/// Common trait for I2C SCL pins.
pub trait I2CSclPin<const N: usize>: I2CPin<N> {
    #[inline(always)]
    fn config(&self) {
        // Reference to the PAD register.
        let _pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, drive output enable from peripheral,
        // drive output from peripheral, select UART function.
        io[1].write(FUNCSEL & 0x1F);

        // Configure the pad.
        // Enable output, disable input, drive to 4 mA, Pull Up, Schmitt, Slew limited.
        //pad.write((1 << 3) | (1 << 1));
    }
}


impl I2CSclPin<0> for Gpio<01> {}
impl I2CSclPin<1> for Gpio<03> {}
impl I2CSclPin<0> for Gpio<05> {}
impl I2CSclPin<1> for Gpio<07> {}
impl I2CSclPin<0> for Gpio<09> {}
impl I2CSclPin<1> for Gpio<11> {}
impl I2CSclPin<0> for Gpio<13> {}
impl I2CSclPin<1> for Gpio<15> {}
impl I2CSclPin<0> for Gpio<17> {}
impl I2CSclPin<1> for Gpio<19> {}
impl I2CSclPin<0> for Gpio<21> {}
impl I2CSclPin<1> for Gpio<23> {}
impl I2CSclPin<0> for Gpio<25> {}
impl I2CSclPin<1> for Gpio<27> {}
impl I2CSclPin<0> for Gpio<29> {}
