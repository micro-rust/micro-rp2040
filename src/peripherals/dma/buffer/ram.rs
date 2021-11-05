//! Safe abstracion over RAM Buffers.
//! Provides validation against malformed slices.
//! The safety checks can be disabled with the `skip-address-safety-checks` feature.



use crate::error::SystemError;

use micro::drivers::Data;

use super::{ Buffer, DestinationBuffer, SourceBuffer };



/// RAM Buffer to copy data from.
pub struct CopyFromRam<'a, D: Data>(&'a [D]);


/// RAM Buffer to copy data into.
pub struct CopyIntoRam<'a, D: Data>(&'a mut [D]);



impl<'a, D: Data> Buffer for CopyFromRam<'a, D> {
    unsafe fn raw(addr: u32, size: usize) -> Self {
        Self( core::slice::from_raw_parts(addr as *const _, size) )
    }

    #[inline(always)]
    fn addr(&self) -> u32 {
        self.0.as_ptr() as u32
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.0.len()
    }
}


impl<'a, D: Data> Buffer for CopyIntoRam<'a, D> {
    unsafe fn raw(addr: u32, size: usize) -> Self {
        Self( core::slice::from_raw_parts_mut(addr as *mut _, size) )
    }

    #[inline(always)]
    fn addr(&self) -> u32 {
        self.0.as_ptr() as u32
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.0.len()
    }
}



impl<'a, D: Data> SourceBuffer<'a, D> for CopyFromRam<'a, D> {
    fn create(buf: &'a [D]) -> Result<Self, SystemError> {
        #[cfg(not(feature = "skip-address-safety-checks"))]
        {
            let (addr, size) = (buf.as_ptr() as u32, buf.len() as u32);

            match addr {
                0x20000000..=0x20041FFF => match addr + size as u32 {
                    0x20000001..=0x20042000 => (),
                    _ => return Err( SystemError::BufferDoesNotFit ),
                },

                0x21000000..=0x2103FFFF => match addr + size as u32 {
                    0x21000001..=0x21040000 => (),
                    _ => return Err( SystemError::BufferDoesNotFit ),
                },

                _ => return Err( SystemError::NotRamRegion ),
            }
        }

        Ok( Self( buf ) )
    }
}


impl<'a, D: Data> DestinationBuffer<'a, D> for CopyIntoRam<'a, D> {
    fn create(buf: &'a mut [D]) -> Result<Self, SystemError> {
        #[cfg(not(feature = "skip-address-safety-checks"))]
        {
            let (addr, size) = (buf.as_ptr() as u32, buf.len() as u32);

            match addr {
                0x20000000..=0x20041FFF => match addr + size as u32 {
                    0x20000001..=0x20042000 => (),
                    _ => return Err( SystemError::BufferDoesNotFit ),
                },

                0x21000000..=0x2103FFFF => match addr + size as u32 {
                    0x21000001..=0x21040000 => (),
                    _ => return Err( SystemError::BufferDoesNotFit ),
                },

                _ => return Err( SystemError::NotRamRegion ),
            }
        }

        Ok( Self( buf ) )
    }
}
