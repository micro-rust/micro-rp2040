//! Copy buffers.
//! Abstraction over a pair of Copy buffers.


use crate::error::SystemError;


use core::marker::PhantomData;


use micro::drivers::Data;


pub struct CopyBuffer<'a, D: Data> {
    /// Source and destination.
    addr: (u32, u32),

    /// Size of the buffers.
    size: usize,

    _ph: PhantomData<&'a [D]>,
}


impl<'a, D: Data> CopyBuffer<'a, D> {
    pub fn create(src: &'a [D], dest: &'a mut [D]) -> Result<Self, SystemError> {
        // Get addresses.
        let addr = (src.as_ptr() as u32, dest.as_ptr() as u32);

        // Get sizes.
        let size = (src.len(), dest.len());

        #[cfg(not(feature = "skip-address-safety-checks"))]
        match addr.0 {
            0x20000000..=0x20041FFF => match addr.0 + size.0 as u32 {
                0x20000001..=0x20042000 => (),
                _ => return Err( SystemError::BufferDoesNotFit ),
            },

            0x21000000..=0x2103FFFF => match addr.0 + size.0 as u32 {
                0x21000001..=0x21040000 => (),
                _ => return Err( SystemError::BufferDoesNotFit ),
            },

            _ => return Err( SystemError::NotRamRegion ),
        }

        #[cfg(not(feature = "skip-address-safety-checks"))]
        match addr.1 {
            0x20000000..=0x20041FFF => match addr.1 + size.1 as u32 {
                0x20000001..=0x20042000 => (),
                _ => return Err( SystemError::BufferDoesNotFit ),
            },

            0x21000000..=0x2103FFFF => match addr.1 + size.1 as u32 {
                0x21000001..=0x21040000 => (),
                _ => return Err( SystemError::BufferDoesNotFit ),
            },

            _ => return Err( SystemError::NotRamRegion ),
        }

        #[cfg(not(feature = "skip-size-safety-checks"))]
        if size.0 != size.1 {
            return Err( SystemError::UnequalBufferSize );
        }

        #[cfg(not(feature = "skip-size-safety-checks"))]
        {
            // Check source overlapping into destination.
            let end0 = addr.0 + size.0 as u32;
            let end1 = addr.1 + size.1 as u32;

            if (end0 > addr.1) && (end0 < end1) {
                return Err( SystemError::BufferOverlap )
            }

            if (end1 > addr.0) && (end1 < end0) {
                return Err( SystemError::BufferOverlap )
            }

            if addr.0 == addr.1 {
                return Err( SystemError::BufferOverlap )
            }
        }

        Ok(Self {
            addr,
            size: size.0,
            _ph: PhantomData,
        })
    }

    /// Returns the address of the read buffer.
    #[inline(always)]
    pub fn read(&self) -> u32 {
        self.addr.0
    }

    /// Returns the address of the write buffer.
    #[inline(always)]
    pub fn write(&self) -> u32 {
        self.addr.1
    }

    /// Returns the size of the buffers.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        self.size as u32
    }
}
