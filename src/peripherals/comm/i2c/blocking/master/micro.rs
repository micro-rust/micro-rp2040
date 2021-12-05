//! `micro` implementations for I2C Master.
//! TODO : Implement timeouts.
//! TODO : Implement lifetime correctly, currently an interface can outlive the instance.


use crate::prelude::*;
use crate::peripherals::comm::i2c::HANDLES;
use crate::sync::Spinlock;

use embedded_hal::i2c::{ SevenBitAddress, TenBitAddress };

use micro::drivers::{ Handle, comm::{ I2CDriver, I2CWrite, I2CWriteRead } };

use super::I2CMasterInterface;

impl<const N: usize> I2CDriver for I2CMasterInterface<N> where[(); 28+N]: Sized {
    type Error = Error;
}


impl<const N: usize> I2CWrite<SevenBitAddress> for I2CMasterInterface<N> where[(); 28+N]: Sized {
    fn write<'a>(&mut self, addr: SevenBitAddress, bytes: &[u8], stop: bool) -> Result<&'a mut Handle, Self::Error> {
        // Wait for lock.
        loop {
            match Spinlock::<{28+N}>::acquire() {
                Ok(_) => {
                    Self::address(addr as u16, true);
                    Self::writehal(bytes, stop)?;

                    // This function is blocking, just set the Handle and finish it.
                    let handle: &mut Handle = unsafe { &mut HANDLES[N] };
                    *handle = Handle::new();
                    handle.launch();
                    handle.done();

                    return Ok( handle );
                },
                _ => continue,
            }
        }
    }
}

impl<const N: usize> I2CWriteRead<SevenBitAddress> for I2CMasterInterface<N> where[(); 28+N]: Sized {
    fn write_read<'a>(&mut self, addr: SevenBitAddress, bytes: &[u8], buffer: &mut [u8], restart: bool) -> Result<&'a mut Handle, Self::Error> {
        // Wait for lock.
        loop {
            match Spinlock::<{28+N}>::acquire() {
                Ok(_) => {
                    Self::address(addr as u16, true);
                    Self::writehal(bytes, !restart)?;
                    Self::readhal(buffer)?;

                    // This function is blocking, just set the Handle and finish it.
                    let handle: &mut Handle = unsafe { &mut HANDLES[N] };
                    *handle = Handle::new();
                    handle.launch();
                    handle.done();

                    return Ok( handle );
                },
                _ => continue,
            }
        }
    }
}

impl<const N: usize> I2CWrite<TenBitAddress> for I2CMasterInterface<N> where[(); 28+N]: Sized {
    fn write<'a>(&mut self, addr: TenBitAddress, bytes: &[u8], stop: bool) -> Result<&'a mut Handle, Self::Error> {
        // Wait for lock.
        loop {
            match Spinlock::<{28+N}>::acquire() {
                Ok(_) => {
                    Self::address(addr as u16, false);
                    Self::writehal(bytes, stop)?;

                    // This function is blocking, just set the Handle and finish it.
                    let handle: &mut Handle = unsafe { &mut HANDLES[N] };
                    *handle = Handle::new();
                    handle.launch();
                    handle.done();

                    return Ok( handle );
                },
                _ => continue,
            }
        }
    }
}

impl<const N: usize> I2CWriteRead<TenBitAddress> for I2CMasterInterface<N> where[(); 28+N]: Sized {
    fn write_read<'a>(&mut self, addr: TenBitAddress, bytes: &[u8], buffer: &mut [u8], restart: bool) -> Result<&'a mut Handle, Self::Error> {
        // Wait for lock.
        loop {
            match Spinlock::<{28+N}>::acquire() {
                Ok(_) => {
                    Self::address(addr as u16, false);
                    Self::writehal(bytes, !restart)?;
                    Self::readhal(buffer)?;

                    // This function is blocking, just set the Handle and finish it.
                    let handle: &mut Handle = unsafe { &mut HANDLES[N] };
                    *handle = Handle::new();
                    handle.launch();
                    handle.done();

                    return Ok( handle );
                },
                _ => continue,
            }
        }
    }
}
