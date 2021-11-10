//! All possible communication errors.




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommError {
    /// UART Error.
    Uart(UartError),

    /// SPI Error.
    Spi(SPIError),

    /// USB Error.
    Usb(USBError),

    /// Unknown / Other error.
    Other,
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UartError {
    /// Data has been lost due to more incoming data while full.
    Overrun,

    /// A bad break condition was detected.
    Break,

    /// The parity of the UART frame was incorrect.
    Parity,

    /// An invalid stop bit was received.
    Stop,

    /// A timeout ocurred while expecting to receive a frame.
    Timeout,

    /// A DMA error ocurred.
    Dma,

    /// Unknown / Other error.
    Other,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPIError {
    /// Receive timeout.
    Timeout,

    /// Receive overrun.
    Overrun,

    /// Unknown / Other error.
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum USBError {
    /// Unknown / Other error.
    Other,
}
