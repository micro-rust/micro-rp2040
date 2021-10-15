//! UART Pin abstractions.


use super::*;



/// UART Function selector.
const FUNCSEL : u32 = 2;


/// UART Pin object. Can only be moved.
pub struct UartPin<const N: u32>;

impl<const N: u32> UartPin<N> {
    #[inline(always)]
    pub const fn from(_: Gpio<N>) -> Self {
        Self
    }
}



impl<const N : u32> PinTrait for UartPin<N> {
    const IO  : u32 = 0x40014000 + {0x08 * N};
    const PAD : u32 = 0x4001C000 + {0x04 * N} + 0x04;
}




/// Common trait for UART 0 pins.
pub trait Uart0Pin {}

/// Common trait for UART 1 pins.
pub trait Uart1Pin {}


impl Uart0Pin for UartPin<0>  {}
impl Uart0Pin for UartPin<1>  {}
impl Uart0Pin for UartPin<2>  {}
impl Uart0Pin for UartPin<3>  {}

impl Uart1Pin for UartPin<4>  {}
impl Uart1Pin for UartPin<5>  {}
impl Uart1Pin for UartPin<6>  {}
impl Uart1Pin for UartPin<7>  {}
impl Uart1Pin for UartPin<8>  {}
impl Uart1Pin for UartPin<9>  {}
impl Uart1Pin for UartPin<10> {}
impl Uart1Pin for UartPin<11> {}

impl Uart0Pin for UartPin<12> {}
impl Uart0Pin for UartPin<13> {}
impl Uart0Pin for UartPin<14> {}
impl Uart0Pin for UartPin<15> {}
impl Uart0Pin for UartPin<16> {}
impl Uart0Pin for UartPin<17> {}
impl Uart0Pin for UartPin<18> {}
impl Uart0Pin for UartPin<19> {}

impl Uart1Pin for UartPin<20> {}
impl Uart1Pin for UartPin<21> {}
impl Uart1Pin for UartPin<22> {}
impl Uart1Pin for UartPin<23> {}
impl Uart1Pin for UartPin<24> {}
impl Uart1Pin for UartPin<25> {}
impl Uart1Pin for UartPin<26> {}
impl Uart1Pin for UartPin<27> {}

impl Uart0Pin for UartPin<28> {}
impl Uart0Pin for UartPin<29> {}




/// Common trait for UART TX pins.
pub trait UartTxPin: PinTrait {
    #[inline(always)]
    fn config(&self) {
        // Reference to the PAD register.
        let pad: &'static mut AtomicRegister<u32> = unsafe { &mut *(Self::PAD as *mut _) };

        // Configure the pad.
        // Enable output, disable input, drive to 4 mA, Pull Up, no Schmitt, Slew fast.
        pad.write((0x1 << 4) | (1 << 3) | 1);

        // Reference to the IO register.
        let io: &'static mut [AtomicRegister<u32>; 2] = unsafe { &mut *(Self::IO as *mut _) };

        // Configure IO mux.
        // No IRQ, don't invert input / output, drive output enable from peripheral,
        // drive output from peripheral, select UART function.
        io[1].write(FUNCSEL & 0x1F);
    }
}


impl UartTxPin for UartPin<0>  {}
impl UartTxPin for UartPin<4>  {}
impl UartTxPin for UartPin<8>  {}
impl UartTxPin for UartPin<12> {}
impl UartTxPin for UartPin<16> {}
impl UartTxPin for UartPin<20> {}
impl UartTxPin for UartPin<24> {}
impl UartTxPin for UartPin<28> {}




/// Common trait for UART RX pins.
pub trait UartRxPin: PinTrait {
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


impl UartRxPin for UartPin<1>  {}
impl UartRxPin for UartPin<5>  {}
impl UartRxPin for UartPin<9>  {}
impl UartRxPin for UartPin<13> {}
impl UartRxPin for UartPin<17> {}
impl UartRxPin for UartPin<21> {}
impl UartRxPin for UartPin<25> {}
impl UartRxPin for UartPin<29> {}




/// Common trait for UART CTS pins.
pub trait UartCtsPin: PinTrait {
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


impl UartCtsPin for UartPin<2>  {}
impl UartCtsPin for UartPin<6>  {}
impl UartCtsPin for UartPin<10> {}
impl UartCtsPin for UartPin<14> {}
impl UartCtsPin for UartPin<18> {}
impl UartCtsPin for UartPin<22> {}
impl UartCtsPin for UartPin<26> {}




/// Common trait for UART CTS pins.
pub trait UartRtsPin: PinTrait {
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


impl UartRtsPin for UartPin<3>  {}
impl UartRtsPin for UartPin<7>  {}
impl UartRtsPin for UartPin<11> {}
impl UartRtsPin for UartPin<15> {}
impl UartRtsPin for UartPin<19> {}
impl UartRtsPin for UartPin<23> {}
impl UartRtsPin for UartPin<27> {}
