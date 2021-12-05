//! Clock configuration module.



pub struct Clocks;


impl Clocks {
    /// Returns the frequency of the system clock.
    #[inline(always)]
    pub fn sysfreq() -> u32 {
        unsafe { crate::sys::CLOCKS[Clock::System.index()] }
    }

    /// Returns the frequency of the peripheral clock.
    #[inline(always)]
    pub fn peripheral() -> u32 {
        unsafe { crate::sys::CLOCKS[Clock::Peripheral.index()] }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clock {
    /// Clock Input 0.
    GPInput0,

    /// Clock Input 1.
    GPInput1,

    /// Ring Oscillator.
    Rosc,

    /// Crystal Oscillator.
    Xosc,

    /// System PLL.
    PllSys,

    /// USB PLL.
    PllUsb,

    /// Reference clock.
    Reference,

    /// System clock.
    System,

    /// ADC clock.
    Adc,

    /// Rtc clock.
    Rtc,

    /// USB clock.
    Usb,

    /// Peripheral clock.
    Peripheral,

    /// General Purpose Output 0.
    GPOutput0,

    /// General Purpose Output 1.
    GPOutput1,

    /// General Purpose Output 2.
    GPOutput2,

    /// General Purpose Output 3.
    GPOutput3,

    /// Auxiliary clock.
    Auxiliary,

    /// No clock source / Initialization state.
    None,
}

impl Clock {
    /// Returns the index of the clock frequency index.
    pub const fn index(&self) -> usize {
        match *self {
            Clock::GPInput0 => 0,
            Clock::GPInput1 => 1,

            Clock::Rosc => 2,
            Clock::Xosc => 3,

            Clock::PllSys => 4,
            Clock::PllUsb => 5,

            Clock::Reference => 6,
            Clock::System => 7,

            Clock::Adc =>  8,
            Clock::Rtc =>  9,
            Clock::Usb => 10,

            Clock::Peripheral => 11,

            Clock::GPOutput0 => 12,
            Clock::GPOutput1 => 13,
            Clock::GPOutput2 => 14,
            Clock::GPOutput3 => 15,

            Clock::Auxiliary => 15,

            Clock::None => 15,
        }
    }

}
