//! SPI Pin traits.


use crate::prelude::*;

use super::*;


/// I2C Function selector.
const FUNCSEL : u32 = 1;


/// Common trait for all SPI pins.
pub trait SPIPin<const N: usize>: PinTrait {}

impl SPIPin<0> for Gpio<00> {}
impl SPIPin<0> for Gpio<01> {}
impl SPIPin<0> for Gpio<02> {}
impl SPIPin<0> for Gpio<03> {}
impl SPIPin<0> for Gpio<04> {}
impl SPIPin<0> for Gpio<05> {}
impl SPIPin<0> for Gpio<06> {}
impl SPIPin<0> for Gpio<07> {}

impl SPIPin<1> for Gpio<08> {}
impl SPIPin<1> for Gpio<09> {}
impl SPIPin<1> for Gpio<10> {}
impl SPIPin<1> for Gpio<11> {}
impl SPIPin<1> for Gpio<12> {}
impl SPIPin<1> for Gpio<13> {}
impl SPIPin<1> for Gpio<14> {}
impl SPIPin<1> for Gpio<15> {}

impl SPIPin<0> for Gpio<16> {}
impl SPIPin<0> for Gpio<17> {}
impl SPIPin<0> for Gpio<18> {}
impl SPIPin<0> for Gpio<19> {}
impl SPIPin<0> for Gpio<20> {}
impl SPIPin<0> for Gpio<21> {}
impl SPIPin<0> for Gpio<22> {}
impl SPIPin<0> for Gpio<23> {}

impl SPIPin<1> for Gpio<24> {}
impl SPIPin<1> for Gpio<25> {}
impl SPIPin<1> for Gpio<26> {}
impl SPIPin<1> for Gpio<27> {}
impl SPIPin<1> for Gpio<28> {}
impl SPIPin<1> for Gpio<29> {}



/// Common trait for SPI MISO pins.
pub trait SPIMisoPin<const N: usize> : SPIPin<N> {
    fn config(&self) {
        // Reference to the PAD register.
        let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, drive output enable from peripheral,
        // drive output from peripheral, select UART function.
        io[1].write(FUNCSEL & 0x1F);

        // Configure the pad.
        // Enable output, disable input, drive to 4 mA, Pull Up, Schmitt, Slew limited.
        pad.write((1 << 6) | (1 << 1));
    }
}

impl SPIMisoPin<0> for Gpio<00> {}
impl SPIMisoPin<0> for Gpio<04> {}

impl SPIMisoPin<1> for Gpio<08> {}
impl SPIMisoPin<1> for Gpio<12> {}

impl SPIMisoPin<0> for Gpio<16> {}
impl SPIMisoPin<0> for Gpio<20> {}

impl SPIMisoPin<1> for Gpio<24> {}
impl SPIMisoPin<1> for Gpio<28> {}



/// Common trait for SPI MOSI pins.
pub trait SPIMosiPin<const N: usize> : SPIPin<N> {
    fn config(&self) {
        // Reference to the PAD register.
        let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, drive output enable from peripheral,
        // drive output from peripheral, select UART function.
        io[1].write(FUNCSEL & 0x1F);

        // Configure the pad.
        // Enable output, disable input, drive to 4 mA, Pull Up, Schmitt, Slew limited.
        pad.write(1 << 1);
    }
}

impl SPIMosiPin<0> for Gpio<03> {}
impl SPIMosiPin<0> for Gpio<07> {}

impl SPIMosiPin<1> for Gpio<11> {}
impl SPIMosiPin<1> for Gpio<15> {}

impl SPIMosiPin<0> for Gpio<19> {}
impl SPIMosiPin<0> for Gpio<23> {}

impl SPIMosiPin<1> for Gpio<27> {}


/// Common trait for SPI SCK pins.
pub trait SPISckPin<const N: usize> : SPIPin<N> {
    fn config(&self) {
        // Reference to the PAD register.
        let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, drive output enable from peripheral,
        // drive output from peripheral, select UART function.
        io[1].write(FUNCSEL & 0x1F);

        // Configure the pad.
        // Enable output, disable input, drive to 4 mA, Pull Up, Schmitt, Slew limited.
        pad.write(1 << 1);
    }
}

impl SPISckPin<0> for Gpio<02> {}
impl SPISckPin<0> for Gpio<06> {}

impl SPISckPin<1> for Gpio<10> {}
impl SPISckPin<1> for Gpio<14> {}

impl SPISckPin<0> for Gpio<18> {}
impl SPISckPin<0> for Gpio<22> {}

impl SPISckPin<1> for Gpio<26> {}




/// Common trait for SPI Master mode CS pin.
pub trait SPICsMasterPin: PinTrait {
    fn config(&self) {
        // Reference to the SIO OUT ENABLE register.
        let oe = unsafe { &mut *(0xD0000020 as *mut SIORegister<u32>) };

        // Reference to the SIO OUT ENABLE register.
        let os = unsafe { &mut *(0xD0000014 as *mut SIORegister<u32>) };

        // Reference to the PAD register.
        let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure the pad.
        pad.write(1 << 3);

        // Configure IO mux.
        // No function, 
        io[1].write(5);

        // Enable output.
        oe.set(1 << self.number());

        // Set high.
        os.write(1 << self.number());
    }

    /// Selects the SPI slave.
    fn select(&self) {
        // Reference to the SIO OUT CLEAR register.
        let oc = unsafe { &mut *(0xD0000018 as *mut SIORegister<u32>) };

        oc.write(1 << self.number());
    }

    /// Deselects the SPI slave.
    fn deselect(&self) {
        // Reference to the SIO OUT SET register.
        let os = unsafe { &mut *(0xD0000014 as *mut SIORegister<u32>) };

        os.write(1 << self.number());
    }
}


impl SPICsMasterPin for Gpio<00> {}
impl SPICsMasterPin for Gpio<01> {}
impl SPICsMasterPin for Gpio<02> {}
impl SPICsMasterPin for Gpio<03> {}
impl SPICsMasterPin for Gpio<04> {}
impl SPICsMasterPin for Gpio<05> {}
impl SPICsMasterPin for Gpio<06> {}
impl SPICsMasterPin for Gpio<07> {}
impl SPICsMasterPin for Gpio<08> {}
impl SPICsMasterPin for Gpio<09> {}
impl SPICsMasterPin for Gpio<10> {}
impl SPICsMasterPin for Gpio<11> {}
impl SPICsMasterPin for Gpio<12> {}
impl SPICsMasterPin for Gpio<13> {}
impl SPICsMasterPin for Gpio<14> {}
impl SPICsMasterPin for Gpio<15> {}
impl SPICsMasterPin for Gpio<16> {}
impl SPICsMasterPin for Gpio<17> {}
impl SPICsMasterPin for Gpio<18> {}
impl SPICsMasterPin for Gpio<19> {}
impl SPICsMasterPin for Gpio<20> {}
impl SPICsMasterPin for Gpio<21> {}
impl SPICsMasterPin for Gpio<22> {}
impl SPICsMasterPin for Gpio<23> {}
impl SPICsMasterPin for Gpio<24> {}
impl SPICsMasterPin for Gpio<25> {}
impl SPICsMasterPin for Gpio<26> {}
impl SPICsMasterPin for Gpio<27> {}
impl SPICsMasterPin for Gpio<28> {}
impl SPICsMasterPin for Gpio<29> {}
