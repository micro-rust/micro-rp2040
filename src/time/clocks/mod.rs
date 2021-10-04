//! Clock configuration module.



/// Clock information module.
mod info;

/// Clock sources.
mod sources;

/// Clock outputs.
mod outputs;


pub use self::info::ClockInfo;
use self::sources::ClockSources;
use self::outputs::ClockOutputs;


pub struct Clocks {
    /// Clock sources of the RP2040.
    sources: ClockSources,

    /// Clock outputs of the RP2040.
    outputs: ClockOutputs,

    /// Array with all the frequencies of the RP2040 clocks.
    freqs: [u32; 16],
}


impl Clocks {
    /// Static initializer.
    #[inline(always)]
    pub const fn empty() -> Clocks {
        Clocks {
            sources: ClockSources::empty(),
            outputs: ClockOutputs::empty(),
            freqs: [0u32; 16],
        }
    }

    /// Initialization code.
    pub fn init(&mut self) {
        self.outputs.preinit();
        self.sources.init();
        self.outputs.init();
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
            GPInput0 => 0,
            GPInput1 => 1,

            Rosc => 2,
            Xosc => 3,

            PllSys => 4,
            PllUsb => 5,

            Reference => 6,
            System => 7,

            Adc =>  8,
            Rtc =>  9,
            Usb => 10,

            Peripheral => 11,

            GPOutput0 => 12,
            GPOutput1 => 13,
            GPOutput2 => 14,
            GPOutput3 => 15,

            Auxiliary => 16,

            Clock::None => 16,
        }
    }

}
