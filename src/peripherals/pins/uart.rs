//! UART Pin abstractions.


use super::*;



/// UART Function selector.
const FUNCSEL : u32 = 2;



/// Common trait for UART pins.
pub trait UartPin<const N: usize>: PinTrait {}


impl UartPin<0> for Gpio<00> {}
impl UartPin<0> for Gpio<01> {}
impl UartPin<0> for Gpio<02> {}
impl UartPin<0> for Gpio<03> {}

impl UartPin<1> for Gpio<04> {}
impl UartPin<1> for Gpio<05> {}
impl UartPin<1> for Gpio<06> {}
impl UartPin<1> for Gpio<07> {}
impl UartPin<1> for Gpio<08> {}
impl UartPin<1> for Gpio<09> {}
impl UartPin<1> for Gpio<10> {}
impl UartPin<1> for Gpio<11> {}

impl UartPin<0> for Gpio<12> {}
impl UartPin<0> for Gpio<13> {}
impl UartPin<0> for Gpio<14> {}
impl UartPin<0> for Gpio<15> {}
impl UartPin<0> for Gpio<16> {}
impl UartPin<0> for Gpio<17> {}
impl UartPin<0> for Gpio<18> {}
impl UartPin<0> for Gpio<19> {}

impl UartPin<1> for Gpio<20> {}
impl UartPin<1> for Gpio<21> {}
impl UartPin<1> for Gpio<22> {}
impl UartPin<1> for Gpio<23> {}
impl UartPin<1> for Gpio<24> {}
impl UartPin<1> for Gpio<25> {}
impl UartPin<1> for Gpio<26> {}
impl UartPin<1> for Gpio<27> {}

impl UartPin<0> for Gpio<28> {}
impl UartPin<0> for Gpio<29> {}




/// Common trait for UART TX pins.
pub trait UartTxPin<const N: usize>: UartPin<N> {
    #[inline(always)]
    fn config(&self) {
        // Reference to the PAD register.
        let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

        // Reference to the IO register.
        let io = unsafe { &mut *(Self::IO as *mut [AtomicRegister<u32>; 2]) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, drive output enable from peripheral,
        // drive output from peripheral, select UART function.
        io[1].write(FUNCSEL & 0x1F);

        // Set pullup first.
        pad.write(1 << 3);

        // Configure the pad.
        // Enable output, disable input, drive to 4 mA, Pull Up, no Schmitt, Slew fast.
        pad.write(0);
    }
}


impl UartTxPin<0> for Gpio<00> {}
impl UartTxPin<1> for Gpio<04> {}
impl UartTxPin<1> for Gpio<08> {}
impl UartTxPin<0> for Gpio<12> {}
impl UartTxPin<0> for Gpio<16> {}
impl UartTxPin<1> for Gpio<20> {}
impl UartTxPin<1> for Gpio<24> {}
impl UartTxPin<0> for Gpio<28> {}




/// Common trait for UART RX pins.
pub trait UartRxPin<const N: usize>: UartPin<N> {
    #[inline(always)]
    fn config(&self) {
        // Reference to the PAD register.
        let pad: &'static mut AtomicRegister<u32> = unsafe { &mut *(Self::PAD as *mut _) };

        // Configure the pad.
        // Disable output, enable input, drive to 4 mA, Pull Up, no Schmitt, Slew fast.
        pad.write((1 << 7) | (1 << 6) | (0x1 << 4) | (1 << 3) | 1);

        // Reference to the IO register.
        let io: &'static mut [AtomicRegister<u32>; 2] = unsafe { &mut *(Self::IO as *mut _) };

        // Configure IO mux.
        // No IRQ, don't invert input, disable output,
        // drive output from peripheral, select UART function.
        io[1].write((0x3 << 12)| (FUNCSEL & 0x1F));
    }
}


impl UartRxPin<0> for Gpio<01> {}
impl UartRxPin<1> for Gpio<05> {}
impl UartRxPin<1> for Gpio<09> {}
impl UartRxPin<0> for Gpio<13> {}
impl UartRxPin<0> for Gpio<17> {}
impl UartRxPin<1> for Gpio<21> {}
impl UartRxPin<1> for Gpio<25> {}
impl UartRxPin<0> for Gpio<29> {}




/// Common trait for UART CTS pins.
pub trait UartCtsPin<const N: usize>: UartPin<N> {
    #[inline(always)]
    fn config(&self) {
        // Reference to the PAD register.
        let pad: &'static mut AtomicRegister<u32> = unsafe { &mut *(Self::PAD as *mut _) };

        // Configure the pad.
        // Disable output, enable input, drive to 4 mA, Pull Up, no Schmitt, Slew fast.
        pad.write((1 << 7) | (1 << 6) | (0x1 << 4) | (1 << 3) | 1);

        // Reference to the IO register.
        let io: &'static mut [AtomicRegister<u32>; 2] = unsafe { &mut *(Self::IO as *mut _) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, drive output enable from peripheral,
        // drive output from peripheral, select UART function.
        io[1].write(FUNCSEL & 0x1F);
    }
}


impl UartCtsPin<0> for Gpio<02> {}
impl UartCtsPin<1> for Gpio<06> {}
impl UartCtsPin<1> for Gpio<10> {}
impl UartCtsPin<0> for Gpio<14> {}
impl UartCtsPin<0> for Gpio<18> {}
impl UartCtsPin<1> for Gpio<22> {}
impl UartCtsPin<1> for Gpio<26> {}




/// Common trait for UART CTS pins.
pub trait UartRtsPin<const N: usize>: UartPin<N> {
    #[inline(always)]
    fn config(&self) {
        // Reference to the PAD register.
        let pad: &'static mut AtomicRegister<u32> = unsafe { &mut *(Self::PAD as *mut _) };

        // Configure the pad.
        // Disable output, enable input, drive to 4 mA, Pull Up, no Schmitt, Slew fast.
        pad.write((0x1 << 4) | (1 << 3) | 1);

        // Reference to the IO register.
        let io: &'static mut [AtomicRegister<u32>; 2] = unsafe { &mut *(Self::IO as *mut _) };

        // Configure IO mux.
        // No IRQ, don't invert input, disable output,
        // drive output from peripheral, select UART function.
        io[1].write(FUNCSEL & 0x1F);
    }
}


impl UartRtsPin<0> for Gpio<03>  {}
impl UartRtsPin<1> for Gpio<07>  {}
impl UartRtsPin<1> for Gpio<11> {}
impl UartRtsPin<0> for Gpio<15> {}
impl UartRtsPin<0> for Gpio<19> {}
impl UartRtsPin<1> for Gpio<23> {}
impl UartRtsPin<1> for Gpio<27> {}



// NULL Pin implementation.
impl UartPin<0> for NULLPIN {}
impl UartPin<1> for NULLPIN {}

impl UartRxPin<0> for NULLPIN {
    fn config(&self) {}
}

impl UartTxPin<0> for NULLPIN {
    fn config(&self) {}
}

impl UartCtsPin<0> for NULLPIN {
    fn config(&self) {}
}

impl UartRtsPin<0> for NULLPIN {
    fn config(&self) {}
}

impl UartRxPin<1> for NULLPIN {
    fn config(&self) {}
}

impl UartTxPin<1> for NULLPIN {
    fn config(&self) {}
}

impl UartCtsPin<1> for NULLPIN {
    fn config(&self) {}
}

impl UartRtsPin<1> for NULLPIN {
    fn config(&self) {}
}
