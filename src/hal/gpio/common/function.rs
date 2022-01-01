//! Alternate function of the GPIO.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AltFunction {
    /// Selects the corresponding alternate function for SPI.
    SPI = 1,

    /// Selects the corresponding alternate function for UART.
    UART = 2,

    /// Selects the corresponding alternate function for I2C.
    I2C = 3,

    /// Selects the corresponding alternate function for PWM.
    PWM = 4,

    /// Selects the corresponding alternate function for SIO / Raw Control.
    SIO = 5,

    /// Selects the corresponding alternate function for PIO0.
    PIO0 = 6,

    /// Selects the corresponding alternate function for PIO1.
    PIO1 = 7,

    /// Selects the corresponding alternate function for External Clock Input/Output.
    CLOCK = 8,

    /// Selects the corresponding alternate function for USB detection.
    USBDETECT = 9,

    /// Selects the corresponding alternate function for USB manipulation.
    USB = 10,
}
