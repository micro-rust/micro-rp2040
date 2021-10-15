//! All possible System errors.



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemError {
    /// The System lock could not be acquired.
    NoSystemLock,

    /// The peripheral is already reserved.
    PeripheralNotAvailable,

    /// The GPIO is already reserved.
    GPIONotAvailable,

    /// The DMA Channel is already reserved.
    DMAChannelNotAvailable,

    /// Unknown / Other error.
    Other,
}
