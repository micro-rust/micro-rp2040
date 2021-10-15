//! GP I/O Pins abstraction.
//! Gives safe access to the pins in the RP2040.

#![deny(warnings)]

#![allow(non_camel_case_types)]



pub mod led;
pub mod uart;


use crate::raw::AtomicRegister;
use crate::sync::Syslock;

use micro::Register;


pub use self::pinout::*;


pub(crate) type NULLPIN = Gpio<31>;


pub struct Gpio<const N: u32>;


impl<const N: u32> Gpio<N> {
    /// Acquires the pin at runtime if available.
    pub fn acquire() -> Option<Self> {
        extern "C" {
            static PINLOCK : u32;
        }

        match Syslock::acquire() {
            Some(_) => match (unsafe { PINLOCK } >> N) & 1 {
                0 => Some(Self),
                _ => None,
            },

            _ => None,
        }
    }

    /// DO NOT USE THIS METHOD.
    pub unsafe fn reserve() -> Self {
        Self
    }
}


/// Common trait for all pins to control state and configuration.
pub trait PinTrait {
    const IO  : u32;
    const PAD : u32;
}


impl<const N: u32> PinTrait for Gpio<N> {
    const IO  : u32 = 0x40014000 + {0x08 * N};
    const PAD : u32 = 0x4001C000 + {0x04 * N} + 0x04;
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DriveStrength {
    MilliAmps_2  = 0x0,
    MilliAmps_4  = 0x1,
    MilliAmps_8  = 0x2,
    MilliAmps_12 = 0x3,
}
























#[cfg(not(feature = "pinout-defined"))]
mod pinout {
    use super::*;

    pub type Gpio0 = Gpio<0>;
    pub type Gpio1 = Gpio<1>;
    pub type Gpio2 = Gpio<2>;
    pub type Gpio3 = Gpio<3>;
    pub type Gpio4 = Gpio<4>;
    pub type Gpio5 = Gpio<5>;
    pub type Gpio6 = Gpio<6>;
    pub type Gpio7 = Gpio<7>;
    pub type Gpio8 = Gpio<8>;
    pub type Gpio9 = Gpio<9>;

    pub type Gpio10 = Gpio<10>;
    pub type Gpio11 = Gpio<11>;
    pub type Gpio12 = Gpio<12>;
    pub type Gpio13 = Gpio<13>;
    pub type Gpio14 = Gpio<14>;
    pub type Gpio15 = Gpio<15>;
    pub type Gpio16 = Gpio<16>;
    pub type Gpio17 = Gpio<17>;
    pub type Gpio18 = Gpio<18>;
    pub type Gpio19 = Gpio<19>;

    pub type Gpio20 = Gpio<20>;
    pub type Gpio21 = Gpio<21>;
    pub type Gpio22 = Gpio<22>;
    pub type Gpio23 = Gpio<23>;
    pub type Gpio24 = Gpio<24>;
    pub type Gpio25 = Gpio<25>;
    pub type Gpio26 = Gpio<26>;
    pub type Gpio27 = Gpio<27>;
    pub type Gpio28 = Gpio<28>;
    pub type Gpio29 = Gpio<29>;
}


#[cfg(feature = "raspberry-pico")]
mod pinout {
    use super::*;

    // Board pins.

    pub type Pin1 = Gpio<0>;
    pub type Pin2 = Gpio<1>;
    pub type Pin4 = Gpio<2>;
    pub type Pin5 = Gpio<3>;
    pub type Pin6 = Gpio<4>;
    pub type Pin7 = Gpio<5>;
    pub type Pin9 = Gpio<6>;

    pub type Pin10 = Gpio<7>;
    pub type Pin11 = Gpio<8>;
    pub type Pin12 = Gpio<9>;

    pub type Pin14 = Gpio<10>;
    pub type Pin15 = Gpio<11>;
    pub type Pin16 = Gpio<12>;
    pub type Pin17 = Gpio<13>;
    pub type Pin19 = Gpio<14>;

    pub type Pin20 = Gpio<15>;
    pub type Pin21 = Gpio<16>;
    pub type Pin22 = Gpio<17>;
    pub type Pin24 = Gpio<18>;
    pub type Pin25 = Gpio<19>;

    pub type Pin26 = Gpio<20>;
    pub type Pin27 = Gpio<21>;
    pub type Pin29 = Gpio<22>;

    pub type PowerSave = Gpio<23>;

    pub type SenseVBus = Gpio<24>;

    pub type LedPin = Gpio<25>;

    pub type Analog0 = Gpio<26>;
    pub type Analog1 = Gpio<27>;
    pub type Analog2 = Gpio<28>;

    pub type AnalogVSys = Gpio<29>;


    // Accessible GPIOs.

    pub type Gpio0 = Gpio<0>;
    pub type Gpio1 = Gpio<1>;
    pub type Gpio2 = Gpio<2>;
    pub type Gpio3 = Gpio<3>;
    pub type Gpio4 = Gpio<4>;
    pub type Gpio5 = Gpio<5>;
    pub type Gpio6 = Gpio<6>;

    pub type Gpio7 = Gpio<7>;
    pub type Gpio8 = Gpio<8>;
    pub type Gpio9 = Gpio<9>;

    pub type Gpio10 = Gpio<10>;
    pub type Gpio11 = Gpio<11>;
    pub type Gpio12 = Gpio<12>;
    pub type Gpio13 = Gpio<13>;
    pub type Gpio14 = Gpio<14>;

    pub type Gpio15 = Gpio<15>;
    pub type Gpio16 = Gpio<16>;
    pub type Gpio17 = Gpio<17>;
    pub type Gpio18 = Gpio<18>;
    pub type Gpio19 = Gpio<19>;

    pub type Gpio20 = Gpio<20>;
    pub type Gpio21 = Gpio<21>;
    pub type Gpio22 = Gpio<22>;

    pub type Gpio26 = Gpio<26>;
    pub type Gpio27 = Gpio<27>;
    pub type Gpio28 = Gpio<28>;
}


#[cfg(feature = "arduino-connect")]
mod pinout {

}



#[cfg(feature = "adafruit-feather")]
mod pinout {
    use super::*;

    // Board pins.

    pub type UartTx = Gpio<0>;
    pub type UartRx = Gpio<1>;

    pub type SdaPin = Gpio<2>;
    pub type SclPin = Gpio<3>;

    pub type Digital4 = Gpio<6>;
    pub type Digital5 = Gpio<7>;
    pub type Digital6 = Gpio<8>;
    pub type Digital9 = Gpio<9>;

    pub type Digital10 = Gpio<10>;
    pub type Digital11 = Gpio<11>;
    pub type Digital12 = Gpio<12>;
    pub type Digital13 = Gpio<13>;

    pub type LedPin = Gpio<13>;

    pub type AddrLed = Gpio<16>;

    pub type SckPin  = Gpio<18>;
    pub type MosiPin = Gpio<19>;
    pub type MisoPin = Gpio<20>;

    pub type Digital24 = Gpio<24>;
    pub type Digital25 = Gpio<25>;

    pub type Analog0 = Gpio<26>;
    pub type Analog1 = Gpio<27>;
    pub type Analog2 = Gpio<28>;
    pub type Analog3 = Gpio<29>;


    // Accessible GPIOs.

    pub type Gpio0 = Gpio<0>;
    pub type Gpio0 = Gpio<1>;

    pub type Gpio0 = Gpio<2>;
    pub type Gpio0 = Gpio<3>;

    pub type Gpio0 = Gpio<6>;
    pub type Gpio0 = Gpio<7>;
    pub type Gpio0 = Gpio<8>;
    pub type Gpio0 = Gpio<9>;

    pub type Gpio10 = Gpio<10>;
    pub type Gpio11 = Gpio<11>;
    pub type Gpio12 = Gpio<12>;
    pub type Gpio13 = Gpio<13>;
    pub type Gpio18 = Gpio<18>;
    pub type Gpio19 = Gpio<19>;

    pub type Gpio20 = Gpio<20>;
    pub type Gpio24 = Gpio<24>;
    pub type Gpio25 = Gpio<25>;
    pub type Gpio26 = Gpio<26>;
    pub type Gpio27 = Gpio<27>;
    pub type Gpio28 = Gpio<28>;
    pub type Gpio29 = Gpio<29>;
}


#[cfg(feature = "adafruit-qtpy")]
mod pinout {
    use super::*;

    // Board pins.

    pub type SdaPin = Gpio<4>;
    pub type SclPin = Gpio<5>;

    pub type UartTx = Gpio<6>;
    pub type UartRx = Gpio<7>;

    pub type UartTx = Gpio<6>;

    pub type MisoPin = Gpio<9>;
    pub type MosiPin = Gpio<10>;

    pub type AddrLedPwr = Gpio<11>;

    pub type AddrLed = Gpio<12>;

    pub type Analog3 = Gpio<26>;
    pub type Analog2 = Gpio<27>;
    pub type Analog1 = Gpio<28>;
    pub type Analog0 = Gpio<29>;

    pub type Digital4 = Gpio<4>;
    pub type Digital5 = Gpio<5>;
    pub type Digital6 = Gpio<6>;
    pub type Digital7 = Gpio<7>;
    pub type Digital8 = Gpio<8>;
    pub type Digital9 = Gpio<9>;

    pub type Digital10 = Gpio<10>;


    // Accessible GPIOs.

    pub type Gpio4 = Gpio<4>;
    pub type Gpio5 = Gpio<5>;
    pub type Gpio6 = Gpio<6>;
    pub type Gpio7 = Gpio<7>;
    pub type Gpio8 = Gpio<8>;
    pub type Gpio9 = Gpio<9>;

    pub type Gpio10 = Gpio<10>;
}

#[cfg(feature = "adafruit-itsybitsy")]
mod pinout {
    use super::*;

    // Board pins.

    pub type UartTx = Gpio<0>;
    pub type UartRx = Gpio<1>;

    pub type SdaPin = Gpio<2>;
    pub type SclPin = Gpio<3>;

    pub type Digital4 = Gpio<4>;

    pub type Digital3 = Gpio<5>;

    pub type Digital7 = Gpio<6>;
    pub type Digital9 = Gpio<7>;

    pub type Digital10 = Gpio<8>;
    pub type Digital11 = Gpio<9>;

    pub type Digital12 = Gpio<10>;
    pub type Digital13 = Gpio<11>;

    pub type LedPin = Gpio<11>;

    pub type Digital2 = Gpio<12>;

    pub type Digital5 = Gpio<14>;

    pub type AddrLedPwr = Gpio<16>;

    pub type AddrLed = Gpio<17>;

    pub type SckPin = Gpio<18>;

    pub type MosiPin = Gpio<19>;
    pub type MisoPin = Gpio<20>;

    pub type Digital24 = Gpio<24>;
    pub type Digital25 = Gpio<25>;

    pub type Analog0 = Gpio<26>;
    pub type Analog1 = Gpio<27>;
    pub type Analog2 = Gpio<28>;
    pub type Analog3 = Gpio<29>;


    // Accessible GPIOs.

    pub type Gpio0 = Gpio<0>;
    pub type Gpio1 = Gpio<1>;
    pub type Gpio2 = Gpio<2>;
    pub type Gpio3 = Gpio<3>;
    pub type Gpio4 = Gpio<4>;
    pub type Gpio5 = Gpio<5>;
    pub type Gpio6 = Gpio<6>;
    pub type Gpio7 = Gpio<7>;
    pub type Gpio8 = Gpio<8>;
    pub type Gpio9 = Gpio<9>;

    pub type Gpio10 = Gpio<10>;
    pub type Gpio11 = Gpio<11>;
    pub type Gpio12 = Gpio<12>;
    pub type Gpio14 = Gpio<14>;
    pub type Gpio18 = Gpio<18>;
    pub type Gpio19 = Gpio<19>;

    pub type Gpio20 = Gpio<20>;
    pub type Gpio24 = Gpio<24>;
    pub type Gpio25 = Gpio<25>;
    pub type Gpio26 = Gpio<26>;
    pub type Gpio27 = Gpio<27>;
    pub type Gpio28 = Gpio<28>;
    pub type Gpio29 = Gpio<29>;
}