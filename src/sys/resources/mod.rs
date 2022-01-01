//! System resources.


mod resource;
mod restart;


pub use self::{
    resource::{ Acquire, Release },
    restart::Restart,
};

pub(crate) use self::{
    resource::{ AcquireSystem, ReleaseSystem, },
    restart::{ PauseSystem, ResumeSystem },
};

// Acquisition state.
// ****************************************************************************

#[link_section = ".systembss0.AVAILABLE"]
pub(self) static mut AVAILABLE : [u32; 3] = [0u32; 3];

// ****************************************************************************

// Running state.
// ****************************************************************************

#[link_section = ".systembss1.RUNNING"]
pub(self) static mut RUNNING : [u32; 2] = [0u32; 2];

// ****************************************************************************


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemResource {
    Rosc,
    Xosc,

    SystemPLL,
    USBPLL,

    PeripheralClock,
    SystemClock,
    USBClock,
    RTCClock,
    ADCClock,

    DMACh0,
    DMACh1,
    DMACh2,
    DMACh3,
    DMACh4,
    DMACh5,
    DMACh6,
    DMACh7,
    DMACh8,
    DMACh9,
    DMACh10,
    DMACh11,

    Watchdog,

    Uart0,
    Uart1,
    Spi0,
    Spi1,
    I2c0,
    I2c1,

    Usb,
    Rtc,
    Adc,

    Timer64,

    Alarm0,
    Alarm1,
    Alarm2,
    Alarm3,

    Pio0,
    Pio1,

    Pio0Sm0,
    Pio0Sm1,
    Pio0Sm2,
    Pio0Sm3,

    Pio1Sm0,
    Pio1Sm1,
    Pio1Sm2,
    Pio1Sm3,

    Pwm0,
    Pwm1,
    Pwm2,
    Pwm3,
    Pwm4,
    Pwm5,
    Pwm6,
    Pwm7,

    Gpio0,
    Gpio1,
    Gpio2,
    Gpio3,
    Gpio4,
    Gpio5,
    Gpio6,
    Gpio7,
    Gpio8,
    Gpio9,

    Gpio10,
    Gpio11,
    Gpio12,
    Gpio13,
    Gpio14,
    Gpio15,
    Gpio16,
    Gpio17,
    Gpio18,
    Gpio19,

    Gpio20,
    Gpio21,
    Gpio22,
    Gpio23,
    Gpio24,
    Gpio25,
    Gpio26,
    Gpio27,
    Gpio28,
    Gpio29,
}

impl SystemResource {
    /// Register of the flag.
    /// Forced to never inline to produce one single lookup table.
    #[inline(never)]
    pub const fn register(&self) -> (usize, usize) {
        use SystemResource::*;

        match *self {
            Rosc => (0, 0),
            Xosc => (0, 1),

            SystemPLL => (0, 4),
            USBPLL    => (0, 5),

            PeripheralClock => (0,  8),
            SystemClock     => (0,  9),
            USBClock        => (0, 10),
            RTCClock        => (0, 11),
            ADCClock        => (0, 12),

            DMACh0  => (0, 13),
            DMACh1  => (0, 14),
            DMACh2  => (0, 15),
            DMACh3  => (0, 16),
            DMACh4  => (0, 17),
            DMACh5  => (0, 18),
            DMACh6  => (0, 19),
            DMACh7  => (0, 20),
            DMACh8  => (0, 21),
            DMACh9  => (0, 22),
            DMACh10 => (0, 23),
            DMACh11 => (0, 24),

            Watchdog => (0, 28),

            Uart0 => (1, 0),
            Uart1 => (1, 1),
            Spi0  => (1, 2),
            Spi1  => (1, 3),
            I2c0  => (1, 4),
            I2c1  => (1, 5),

            Usb  => (1, 6),
            Rtc  => (1, 7),
            Adc  => (1, 8),

            Timer64 => (1, 9),

            Alarm0 => (1, 10),
            Alarm1 => (1, 11),
            Alarm2 => (1, 12),
            Alarm3 => (1, 13),

            Pio0 => (1, 14),
            Pio1 => (1, 15),

            Pio0Sm0 => (1, 16),
            Pio0Sm1 => (1, 17),
            Pio0Sm2 => (1, 18),
            Pio0Sm3 => (1, 19),

            Pio1Sm0 => (1, 20),
            Pio1Sm1 => (1, 21),
            Pio1Sm2 => (1, 22),
            Pio1Sm3 => (1, 23),

            Pwm0 => (1, 24),
            Pwm1 => (1, 25),
            Pwm2 => (1, 26),
            Pwm3 => (1, 27),
            Pwm4 => (1, 28),
            Pwm5 => (1, 29),
            Pwm6 => (1, 30),
            Pwm7 => (1, 31),

            Gpio0 => (2, 0),
            Gpio1 => (2, 1),
            Gpio2 => (2, 2),
            Gpio3 => (2, 3),
            Gpio4 => (2, 4),
            Gpio5 => (2, 5),
            Gpio6 => (2, 6),
            Gpio7 => (2, 7),
            Gpio8 => (2, 8),
            Gpio9 => (2, 9),

            Gpio10 => (2, 10),
            Gpio11 => (2, 11),
            Gpio12 => (2, 12),
            Gpio13 => (2, 13),
            Gpio14 => (2, 14),
            Gpio15 => (2, 15),
            Gpio16 => (2, 16),
            Gpio17 => (2, 17),
            Gpio18 => (2, 18),
            Gpio19 => (2, 19),

            Gpio20 => (2, 20),
            Gpio21 => (2, 21),
            Gpio22 => (2, 22),
            Gpio23 => (2, 23),
            Gpio24 => (2, 24),
            Gpio25 => (2, 25),
            Gpio26 => (2, 26),
            Gpio27 => (2, 27),
            Gpio28 => (2, 28),
            Gpio29 => (2, 29),
        }
    }

    /// Returns the masks for the dependencies of this System Resource.
    /// Forced to never inline to produce one single lookup table.
    #[inline(never)]
    pub const fn parents(&self) -> (u32, u32) {
        use SystemResource::*;

        match *self {
            Rosc => (0, 0),
            Xosc => (0, 0),

            SystemPLL => (0x00000003, 0),
            USBPLL    => (0x00000003, 0),

            PeripheralClock => (0x00000012, 0),
            SystemClock     => (0x00000012, 0),
            USBClock        => (0x00000022, 0),
            RTCClock        => (0x00000022, 0),
            ADCClock        => (0x00000022, 0),

            DMACh0  => (0x00000200, 0),
            DMACh1  => (0x00000200, 0),
            DMACh2  => (0x00000200, 0),
            DMACh3  => (0x00000200, 0),
            DMACh4  => (0x00000200, 0),
            DMACh5  => (0x00000200, 0),
            DMACh6  => (0x00000200, 0),
            DMACh7  => (0x00000200, 0),
            DMACh8  => (0x00000200, 0),
            DMACh9  => (0x00000200, 0),
            DMACh10 => (0x00000200, 0),
            DMACh11 => (0x00000200, 0),

            Watchdog => (0x00000002, 0),

            Uart0 => (0x00000300, 0),
            Uart1 => (0x00000300, 0),
            Spi0  => (0x00000300, 0),
            Spi1  => (0x00000300, 0),
            I2c0  => (0x00000200, 0),
            I2c1  => (0x00000200, 0),

            Usb  => (0x00000422, 0),
            Rtc  => (0x00000822, 0),
            Adc  => (0x00001022, 0),

            Timer64 => (0x10000002, 0),

            Alarm0 => (0x10000002, 0x00000200),
            Alarm1 => (0x10000002, 0x00000200),
            Alarm2 => (0x10000002, 0x00000200),
            Alarm3 => (0x10000002, 0x00000200),

            Pio0 => (0x00000200, 0),
            Pio1 => (0x00000200, 0),

            Pio0Sm0 => (0x00000200, 0x00004000),
            Pio0Sm1 => (0x00000200, 0x00004000),
            Pio0Sm2 => (0x00000200, 0x00004000),
            Pio0Sm3 => (0x00000200, 0x00004000),

            Pio1Sm0 => (0x00000200, 0x00008000),
            Pio1Sm1 => (0x00000200, 0x00008000),
            Pio1Sm2 => (0x00000200, 0x00008000),
            Pio1Sm3 => (0x00000200, 0x00008000),

            Pwm0 => (0x00000200, 0),
            Pwm1 => (0x00000200, 0),
            Pwm2 => (0x00000200, 0),
            Pwm3 => (0x00000200, 0),
            Pwm4 => (0x00000200, 0),
            Pwm5 => (0x00000200, 0),
            Pwm6 => (0x00000200, 0),
            Pwm7 => (0x00000200, 0),

            Gpio0 => (0, 0),
            Gpio1 => (0, 0),
            Gpio2 => (0, 0),
            Gpio3 => (0, 0),
            Gpio4 => (0, 0),
            Gpio5 => (0, 0),
            Gpio6 => (0, 0),
            Gpio7 => (0, 0),
            Gpio8 => (0, 0),
            Gpio9 => (0, 0),

            Gpio10 => (0, 0),
            Gpio11 => (0, 0),
            Gpio12 => (0, 0),
            Gpio13 => (0, 0),
            Gpio14 => (0, 0),
            Gpio15 => (0, 0),
            Gpio16 => (0, 0),
            Gpio17 => (0, 0),
            Gpio18 => (0, 0),
            Gpio19 => (0, 0),

            Gpio20 => (0, 0),
            Gpio21 => (0, 0),
            Gpio22 => (0, 0),
            Gpio23 => (0, 0),
            Gpio24 => (0, 0),
            Gpio25 => (0, 0),
            Gpio26 => (0, 0),
            Gpio27 => (0, 0),
            Gpio28 => (0, 0),
            Gpio29 => (0, 0),
        }
    }

    /// Returns the masks for the dependants of this System Resource.
    /// Forced to never inline to produce one single lookup table.
    #[inline(never)]
    pub const fn children(&self) -> (u32, u32) {
        use SystemResource::*;

        match *self {
            Rosc => (0, 0),
            Xosc => (0x10000030, 0),

            SystemPLL => (0x00000300, 0),
            USBPLL    => (0x00001C00, 0),

            PeripheralClock => (0, 0x00000000),
            SystemClock     => (0x00000000, 0x00000000),
            USBClock        => (0, 0x00000040),
            RTCClock        => (0, 0x00000080),
            ADCClock        => (0, 0x00000100),

            DMACh0  => (0, 0),
            DMACh1  => (0, 0),
            DMACh2  => (0, 0),
            DMACh3  => (0, 0),
            DMACh4  => (0, 0),
            DMACh5  => (0, 0),
            DMACh6  => (0, 0),
            DMACh7  => (0, 0),
            DMACh8  => (0, 0),
            DMACh9  => (0, 0),
            DMACh10 => (0, 0),
            DMACh11 => (0, 0),

            Watchdog => (0, 0x00000000),

            Uart0 => (0, 0),
            Uart1 => (0, 0),
            Spi0  => (0, 0),
            Spi1  => (0, 0),
            I2c0  => (0, 0),
            I2c1  => (0, 0),

            Usb  => (0, 0),
            Rtc  => (0, 0),
            Adc  => (0, 0),

            Timer64 => (0, 0x00000000),

            Alarm0 => (0, 0),
            Alarm1 => (0, 0),
            Alarm2 => (0, 0),
            Alarm3 => (0, 0),

            Pio0 => (0, 0x00000000),
            Pio1 => (0, 0x00000000),

            Pio0Sm0 => (0, 0),
            Pio0Sm1 => (0, 0),
            Pio0Sm2 => (0, 0),
            Pio0Sm3 => (0, 0),

            Pio1Sm0 => (0, 0),
            Pio1Sm1 => (0, 0),
            Pio1Sm2 => (0, 0),
            Pio1Sm3 => (0, 0),

            Pwm0 => (0, 0),
            Pwm1 => (0, 0),
            Pwm2 => (0, 0),
            Pwm3 => (0, 0),
            Pwm4 => (0, 0),
            Pwm5 => (0, 0),
            Pwm6 => (0, 0),
            Pwm7 => (0, 0),

            Gpio0 => (0, 0),
            Gpio1 => (0, 0),
            Gpio2 => (0, 0),
            Gpio3 => (0, 0),
            Gpio4 => (0, 0),
            Gpio5 => (0, 0),
            Gpio6 => (0, 0),
            Gpio7 => (0, 0),
            Gpio8 => (0, 0),
            Gpio9 => (0, 0),

            Gpio10 => (0, 0),
            Gpio11 => (0, 0),
            Gpio12 => (0, 0),
            Gpio13 => (0, 0),
            Gpio14 => (0, 0),
            Gpio15 => (0, 0),
            Gpio16 => (0, 0),
            Gpio17 => (0, 0),
            Gpio18 => (0, 0),
            Gpio19 => (0, 0),

            Gpio20 => (0, 0),
            Gpio21 => (0, 0),
            Gpio22 => (0, 0),
            Gpio23 => (0, 0),
            Gpio24 => (0, 0),
            Gpio25 => (0, 0),
            Gpio26 => (0, 0),
            Gpio27 => (0, 0),
            Gpio28 => (0, 0),
            Gpio29 => (0, 0),
        }
    }
}
