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

    /// The memory region is not in RAM.
    NotRamRegion,

    /// The buffer does not fit in RAM.
    BufferDoesNotFit,

    /// Two buffers are not of the same size.
    UnequalBufferSize,

    BufferOverlap,

    LockUnavailable,

    /// Unknown / Other error.
    Other,
}
