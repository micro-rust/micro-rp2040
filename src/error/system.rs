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

    /// This peripheral can't pause because a running peripheral depends on it.
    PauseDependenciesUnresolved,

    /// This resource is currentl in use.
    ResourceCurrentlyUsed,

    /// This peripehral can't resume execution before the peripherals it
    /// depends on have resumed.
    ResumeDependenciesUnresolved,

    /// The resource is already released.
    ResourceAlreadyReleased,

    /// The necessary dependencies to transition into this state are not met.
    UnresolvedDependencies,

    /// Unknown / Other error.
    Other,
}
