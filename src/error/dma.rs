//! All possible DMA Errors.



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMAError {
    /// An AHB Bus error ocurred.
    AHBError,

    /// A Bus read error ocurred.
    ReadError,

    /// A Bus write error ocurred.
    WriteError,

    /// DMA Control block violates memory bounds.
    CtrlBlockMemoryBounds,

    /// DMA Control block is incompatible with Stream.
    CtrlBlockIncompatible,

    /// The Stream cannot resume, usually due to an error.
    StreamCannotResume,

    /// The given buffers have unequal size.
    UnequalBufferSize,

    /// Unknown / Other error.
    Other,
}

