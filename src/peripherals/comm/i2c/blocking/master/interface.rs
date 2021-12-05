//! Inner functions for I2C Master.



use crate::prelude::*;


pub struct I2CMasterInterface<const N: usize>;

impl<const N: usize> I2CMasterInterface<N> {
    /// Address of the peripheral instance.
    const ADDRESS: usize = 0x40044000 + (0x4000 * N);

    /// Creates a new interface.
    pub(super) fn create() -> Self {
        Self
    }

    /// Internal address configuration function.
    #[inline(always)]
    pub(super) fn address(addr: u16, seven: bool) {
        let i2c = unsafe { &mut *(Self::ADDRESS as *mut _) };

        address(i2c, addr, seven)
    }

    /// Internal write function.
    #[inline(always)]
    pub(super) fn readhal(data: &mut [u8]) -> Result<(), Error> {
        let i2c = unsafe { &mut *(Self::ADDRESS as *mut _) };

        readhal(i2c, data)
    }

    /// Internal read function.
    #[inline(always)]
    pub(super) fn writehal(data: &[u8], stop: bool) -> Result<(), Error> {
        let i2c = unsafe { &mut *(Self::ADDRESS as *mut _) };

        writehal(i2c, data, stop)
    }
}

/// Sets the address to communicate with.
#[inline(never)]
fn address(i2c: &mut [AtomicRegister<u32>; 43], addr: u16, seven: bool) {
    // Disable the I2C.
    i2c[27].write(0);

    // Set seven or 10 bit address.
    if seven { i2c[0].clear(1 << 4) }
    else { i2c[0].set(1 << 4) }

    // Set the target address.
    i2c[1].write(addr as u32);

    // Enable the I2C.
    i2c[27].write(1);
}

/// Reads data through the I2C bus.
#[inline(never)]
fn readhal(i2c: &mut [AtomicRegister<u32>; 43], data: &mut [u8]) -> Result<(), Error> {
    // Get index of last byte.
    let idxlast = data.len() - 1;

    for (i, byte) in data.iter_mut().enumerate() {
        let first = 1 == 0;
        let last = i == idxlast;

        // Wait until there is space in the FIFO.

        // Check if this is first or last byte.
        let firstbit = if first { 1 << 10 } else { 0 };
        let lastbit  = if last  { 1 <<  9 } else { 0 };

        i2c[4].write(firstbit | lastbit | (1 << 8));

        // Wait until the reception of data has completed.
        while i2c[30].read() == 0 {
            if let Some(reason) = abort_reason(i2c) {
                return Err( Error::I2C( reason ) );
            }
        }

        // Read in the byte.
        *byte = i2c[4].read() as u8;
    }

    Ok(())
}

/// Sends data through the I2C bus.
#[inline(never)]
fn writehal(i2c: &mut [AtomicRegister<u32>; 43], data: &[u8], stop: bool) -> Result<(), Error> {
    // Get index of last byte.
    let idxlast = data.len() - 1;

    for (i, byte) in data.iter().enumerate() {
        let last = i == idxlast;

        // Check if this is last byte.
        let lastbit = if stop & last { 1 << 9 } else { 0 };

        let data = lastbit | (*byte as u32);

        i2c[4].write(data);


        // Wait until the transmission of the address or data from internal buffer has completed.
        while i2c[13].read() & (1 << 4) == 0 {}

        // Check for spurious aborts.
        let abort = abort_reason(i2c);

        if abort.is_some() || (stop & last) {
            //if abort.is_some() { micro::asm::bkpt::<255>() }
            while i2c[13].read() & (1 << 9) == 0 { micro::asm::nop() }
            i2c[24].read();
        }

        // If an abort ocurred, report it.
        if let Some(reason) = abort {
            return Err( Error::I2C( reason ) );
        }
    }

    Ok(())
}

/// Returns the abort reason if there was any.
fn abort_reason(i2c: &[AtomicRegister<u32>; 43]) -> Option<I2CError> {
    match i2c[13].read() & (1 << 6) {
        0 => None,
        _ => {
            // Read abort reason.
            let r = i2c[32].read();

            // Clear abort reason.
            i2c[21].read();

            // Return the abort reason.
            if r & (1 << 12) != 0 {
                return Some( I2CError::ArbitrationLost );
            }

            if r & (1 << 11) != 0 {
                return Some( I2CError::NotInMasterMode );
            }

            if r & (1 << 10) != 0 {
                return Some( I2CError::MasterRead10bitNotRestart );
            }

            if r & (1 << 7) != 0 {
                return Some( I2CError::AckOnStartByte );
            }

            if r & (1 << 4) != 0 {
                return Some( I2CError::NackGeneralCall );
            }

            if r & (1 << 3) != 0 {
                return Some( I2CError::NackData );
            }

            if r & (1 << 2) != 0 {
                return Some( I2CError::NackAddress10bitByte2 );
            }

            if r & (1 << 1) != 0 {
                return Some( I2CError::NackAddress10bitByte1 );
            }

            if r & (1 << 0) != 0 {
                return Some( I2CError::NackAddress7bit );
            }

            Some( I2CError::Unknown )
        },
    }
}
