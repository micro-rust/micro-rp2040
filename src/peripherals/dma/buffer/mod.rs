//! Buffer module.
//! Abstractions over all possible buffers that can be used.


mod ram;


use crate::error::SystemError;

use micro::drivers::Data;


pub use self::ram::{ CopyFromRam, CopyIntoRam };


/// Trait for buffers that can be a source of data.
pub trait SourceBuffer<'a, D: Data>: Buffer {
    /// Creates a validated buffer.
    fn create(buf: &'a [D]) -> Result<Self, SystemError>;
}

/// Trait for buffers that can be a destination for data.
pub trait DestinationBuffer<'a, D: Data>: Buffer {
    /// Creates a validated buffer.
    fn create(buf: &'a mut [D]) -> Result<Self, SystemError>;
}

/// Trait for buffers in a peripheral that can consume data.
pub trait ConsumeBuffer: Buffer {}

/// Trait for buffers in a peripheral that can feed data.
pub trait FeedBuffer: Buffer {}

/// Trait for all buffers.
pub trait Buffer: Sized {
    /// Creates an unvalidated buffer.
    unsafe fn raw(addr: u32, size: usize) -> Self;

    /// Returns the base address of the buffer.
    fn addr(&self) -> u32;

    /// Returns the size of the buffer.
    fn size(&self) -> usize;
}