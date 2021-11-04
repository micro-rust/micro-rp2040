//! Buffer module.
//! Abstractions over all possible buffers that can be used.


use crate::error::SystemError;

use micro::drivers::Data;


mod copy;


pub use self::copy::CopyBuffer;


/// Common trait for all Buffers.
pub trait Buffer<D: Data>: Sized {
    /// Creates and validates a DMA Buffer from a given buffer.
    fn create(buf: &[D]) -> Result<Self, SystemError> {
        Self::raw( buf.as_ptr() as u32, buf.len() as u32 )
    }

    /// Creates and validates a DMA Buffer from a buffer raw parts.
    fn raw(ptr: u32, len: u32) -> Result<Self, SystemError>;

    /// Returns the address of the buffer.
    fn address(&self) -> u32;

    /// Returns the address of the buffer.
    fn ptr(&self) -> u32;

    /// Returns the size of the buffer.
    fn size(&self) -> u32;

    /// Returns the size of the buffer.
    fn len(&self) -> u32;
}
