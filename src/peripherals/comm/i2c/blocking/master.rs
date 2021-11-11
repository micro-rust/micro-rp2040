//! Blocking master interface of an I2C instance.
//! TODO : Implement resource acquisition and dropping.

use crate::{
    error::{ Error, I2CError }, peripherals::pins::i2c::*,
    raw::AtomicRegister, sync::Spinlock, sys::clocks::Clocks,
    math::UInt32,
};

use embedded_hal::i2c::{
    SevenBitAddress, TenBitAddress,
    blocking::{ Read, Write, WriteRead },
};

use micro::Register;

use super::super::{ I2CInstance, I2CConfig };


pub struct I2CMaster<const N: usize, SDA: I2CSdaPin<N>, SCL: I2CSclPin<N>> {
    /// SDA Pin.
    sda: SDA,

    /// SCL Pin.
    scl: SCL,
}

impl<const N: usize, SDA: I2CSdaPin<N>, SCL: I2CSclPin<N>> I2CMaster<N, SDA, SCL> where [(); 28 + N]: Sized {
    /// Associated Spinlock.
    type LOCK = Spinlock<{28 + N}>;
}


impl<const N: usize, SDA: I2CSdaPin<N>, SCL: I2CSclPin<N>> I2CMaster<N, SDA, SCL> {
    /// Address of the I2C instance's register block.
    const ADDRESS: usize = 0x40044000 + (0x4000 * N);

    /// Creates an I2C Master from the given I2C instance.
    #[inline(never)]
    pub fn create(instance: I2CInstance<N>, cfg: I2CConfig, sda: SDA, scl: SCL) -> Self {
        use core::mem::forget;

        // Get reference to the I2C instance.
        let i2c = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 43]) };

        // Disable the I2C.
        i2c[27].write(0);

        // Configure the I2C.
        i2c[0].write(
            // Fast mode.
            (0x2 << 1) |
            // Master mode.
            (1 << 0) |
            // Disable slave.
            (1 << 6) |
            // Restart enabled.
            (1 << 5) |
            // TX Empty control enabled.
            (1 << 8)
        );

        // Clear FIFO threshold.
        i2c[14].write(0);
        i2c[15].write(0);

        // Get system frequency.
        let sysfreq = UInt32::new( Clocks::sysfreq() );

        // Get period.
        let period = (sysfreq + UInt32::new(cfg.baud) / 2u32) / cfg.baud;
        let hcnt = (period * 2u32) / 5u32;
        let lcnt = period - hcnt;

        assert!(hcnt <= 0xffff);
        assert!(lcnt <= 0xffff);
        assert!(hcnt >= 8);
        assert!(lcnt >= 8);

        // Calculate TX hold count.
        let txhold = if cfg.baud < 1000000 {
            ((sysfreq * 3u32) / 10000000u32) + 1u32
        } else {
            ((sysfreq * 3u32) / 25000000u32) + 1u32
        };

        assert!(txhold <= lcnt - 2u32);

        // Write the latency and clock registers.
        i2c[ 7].write( u32::from( hcnt ) );
        i2c[ 8].write( u32::from( lcnt ) );
        i2c[31].write( u32::from( txhold ) );

        // Calculate spike suppression length.
        let spklen = if lcnt < 16 { UInt32::new(1) } else { lcnt / 16u32 };
        i2c[40].write( u32::from(spklen) );

        // Enable the I2C block.
        i2c[27].write(1);

        // Configure the pins.
        sda.config();
        scl.config();

        // Destroy the I2C instance to avoid creating more Masters or Slaves.
        forget(instance);

        Self { sda, scl }
    }
}


impl<const N: usize, SDA: I2CSdaPin<N>, SCL: I2CSclPin<N>> Drop for I2CMaster<N, SDA,SCL> {
    fn drop(&mut self) {
        // Release the resource.

        // Reset the peripheral.

    }
}


impl<const N: usize, SDA: I2CSdaPin<N>, SCL: I2CSclPin<N>> Read<SevenBitAddress> for I2CMaster<N, SDA, SCL> {
    type Error = crate::error::Error;

    fn read(&mut self, addr: SevenBitAddress, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let i2c = unsafe { &mut *(Self::ADDRESS as *mut _) };

        // Set the target address.
        address(i2c, addr as u16);

        // Perform the operations.
        readhal(i2c, buffer)
    }
}

impl<const N: usize, SDA: I2CSdaPin<N>, SCL: I2CSclPin<N>> Write<SevenBitAddress> for I2CMaster<N, SDA, SCL> {
    type Error = crate::error::Error;

    fn write(&mut self, addr: SevenBitAddress, bytes: &[u8]) -> Result<(), Self::Error> {
        let i2c = unsafe { &mut *(Self::ADDRESS as *mut _) };

        // Set the target address.
        address(i2c, addr as u16);

        // Perform the operations.
        writehal(i2c, bytes, true)
    }
}

impl<const N: usize, SDA: I2CSdaPin<N>, SCL: I2CSclPin<N>> WriteRead<SevenBitAddress> for I2CMaster<N, SDA, SCL> {
    type Error = crate::error::Error;

    fn write_read(&mut self, addr: SevenBitAddress, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        let i2c = unsafe { &mut *(Self::ADDRESS as *mut _) };

        // Set the target address.
        address(i2c, addr as u16);

        // Perform the operations.
        writehal(i2c, bytes, false)?;
        readhal(i2c, buffer)
    }
}

impl<const N: usize, SDA: I2CSdaPin<N>, SCL: I2CSclPin<N>> Read<TenBitAddress> for I2CMaster<N, SDA, SCL> {
    type Error = crate::error::Error;

    fn read(&mut self, addr: TenBitAddress, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let i2c = unsafe { &mut *(Self::ADDRESS as *mut _) };

        // Set the target address.
        address(i2c, addr as u16);

        // Perform the operations.
        readhal(i2c, buffer)
    }
}

impl<const N: usize, SDA: I2CSdaPin<N>, SCL: I2CSclPin<N>> Write<TenBitAddress> for I2CMaster<N, SDA, SCL> {
    type Error = crate::error::Error;

    fn write(&mut self, addr: TenBitAddress, bytes: &[u8]) -> Result<(), Self::Error> {
        let i2c = unsafe { &mut *(Self::ADDRESS as *mut _) };

        // Set the target address.
        address(i2c, addr as u16);

        // Perform the operations.
        writehal(i2c, bytes, true)
    }
}

impl<const N: usize, SDA: I2CSdaPin<N>, SCL: I2CSclPin<N>> WriteRead<TenBitAddress> for I2CMaster<N, SDA, SCL> {
    type Error = crate::error::Error;

    fn write_read(&mut self, addr: TenBitAddress, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        let i2c = unsafe { &mut *(Self::ADDRESS as *mut _) };

        // Set the target address.
        address(i2c, addr as u16);

        // Perform the operations.
        writehal(i2c, bytes, false)?;
        readhal(i2c, buffer)
    }
}


/// Sets the address to communicate with.
#[inline(always)]
fn address(i2c: &mut [AtomicRegister<u32>; 43], addr: u16) {
    // Disable the I2C.
    i2c[27].write(0);

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
            if abort.is_some() { micro::asm::bkpt::<255>() }
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

            micro::asm::bkpt::<255>();

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
