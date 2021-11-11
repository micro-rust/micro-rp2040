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


impl<'a, D: Data> SourceBuffer<'a, D> for &'a [D] {
    fn create(buf: &'a [D]) -> Result<Self, SystemError> {
        Ok( buf )
    }
}

impl<'a, D: Data> DestinationBuffer<'a, D> for &'a mut [D] {
    fn create(buf: &'a mut [D]) -> Result<Self, SystemError> {
        Ok( buf )
    }
}

impl<'a, D: Data> ConsumeBuffer for &'a [D] {}

impl<'a, D: Data> FeedBuffer for &'a mut [D] {}

impl<'a, D: Data> Buffer for &'a [D] {
    unsafe fn raw(addr: u32, size: usize) -> Self {
        core::slice::from_raw_parts(addr as *const _, size)
    }

    fn addr(&self) -> u32 {
        self.as_ptr() as u32
    }

    fn size(&self) -> usize {
        self.len()
    }
}

impl<'a, D: Data> Buffer for &'a mut [D] {
    unsafe fn raw(addr: u32, size: usize) -> Self {
        core::slice::from_raw_parts_mut(addr as *mut _, size)
    }

    fn addr(&self) -> u32 {
        self.as_ptr() as u32
    }

    fn size(&self) -> usize {
        self.len()
    }
}
