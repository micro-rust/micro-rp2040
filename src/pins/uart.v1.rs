//! UART Pin abstractions.


use super::*;



/// GPIO Function Selector.
const FUNCSEL : u32 = 2;


/// Trait for all UART Pins.
pub trait UartPin<const N: u32, const ID: u32> : PinTrait<N> {}

impl UartPin<0, 0> for Gpio<0> {}
impl UartPin<1, 0> for Gpio<1> {}
impl UartPin<2, 0> for Gpio<2> {}
impl UartPin<3, 0> for Gpio<3> {}

impl UartPin< 4, 1> for Gpio<4>  {}
impl UartPin< 5, 1> for Gpio<5>  {}
impl UartPin< 6, 1> for Gpio<6>  {}
impl UartPin< 7, 1> for Gpio<7>  {}
impl UartPin< 8, 1> for Gpio<8>  {}
impl UartPin< 9, 1> for Gpio<9>  {}
impl UartPin<10, 1> for Gpio<10> {}
impl UartPin<11, 1> for Gpio<11> {}

impl UartPin<12, 0> for Gpio<12> {}
impl UartPin<13, 0> for Gpio<13> {}
impl UartPin<14, 0> for Gpio<14> {}
impl UartPin<15, 0> for Gpio<15> {}
impl UartPin<16, 0> for Gpio<16> {}
impl UartPin<17, 0> for Gpio<17> {}
impl UartPin<18, 0> for Gpio<18> {}
impl UartPin<19, 0> for Gpio<19> {}

impl UartPin<20, 1> for Gpio<20> {}
impl UartPin<21, 1> for Gpio<21> {}
impl UartPin<22, 1> for Gpio<22> {}
impl UartPin<23, 1> for Gpio<23> {}
impl UartPin<24, 1> for Gpio<24> {}
impl UartPin<25, 1> for Gpio<25> {}
impl UartPin<26, 1> for Gpio<26> {}
impl UartPin<27, 1> for Gpio<27> {}

impl UartPin<28, 0> for Gpio<28> {}
impl UartPin<29, 0> for Gpio<29> {}




/// Trait for UART RX pins.
pub trait UartRxPin<const N: u32, const ID: u32> : UartPin<N, ID> {
	/// Configures the pin as an UART input.
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


impl UartRxPin< 1, 0> for Gpio< 1> {}
impl UartRxPin< 5, 1> for Gpio< 5> {}
impl UartRxPin< 9, 1> for Gpio< 9> {}
impl UartRxPin<13, 0> for Gpio<13> {}
impl UartRxPin<17, 0> for Gpio<17> {}
impl UartRxPin<21, 1> for Gpio<21> {}
impl UartRxPin<25, 1> for Gpio<25> {}
impl UartRxPin<29, 0> for Gpio<29> {}
