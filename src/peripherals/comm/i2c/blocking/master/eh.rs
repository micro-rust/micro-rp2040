//! `embedded-hal` implementations for I2C Master.


use crate::prelude::*;
use crate::sync::Spinlock;

use embedded_hal::i2c::{
    SevenBitAddress, TenBitAddress,
    blocking::{ Write, WriteRead },
};

use super::I2CMasterInterface;


impl Write<SevenBitAddress> for I2CMasterInterface<0> {
    type Error = Error;

    fn write(&mut self, addr: SevenBitAddress, bytes: &[u8]) -> Result<(), Self::Error> {
        // Wait for lock.
        loop {
            match Spinlock::<28>::acquire() {
                Ok(_) => {
                    Self::address(addr as u16, true);
                    return Self::writehal(bytes, true)
                },
                _ => continue,
            }
        }
    }
}

impl WriteRead<SevenBitAddress> for I2CMasterInterface<0> {
    type Error = Error;

    fn write_read(&mut self, addr: SevenBitAddress, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        // Wait for lock.
        loop {
            match Spinlock::<28>::acquire() {
                Ok(_) => {
                    Self::address(addr as u16, true);
                    Self::writehal(bytes, false)?;
                    return Self::readhal(buffer)
                },
                _ => continue,
            }
        }
    }
}

impl Write<TenBitAddress> for I2CMasterInterface<0> {
    type Error = Error;

    fn write(&mut self, addr: TenBitAddress, bytes: &[u8]) -> Result<(), Self::Error> {
        // Wait for lock.
        loop {
            match Spinlock::<28>::acquire() {
                Ok(_) => {
                    Self::address(addr as u16, false);
                    return Self::writehal(bytes, true)
                },
                _ => continue,
            }
        }
    }
}

impl WriteRead<TenBitAddress> for I2CMasterInterface<0> {
    type Error = Error;

    fn write_read(&mut self, addr: TenBitAddress, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        // Wait for lock.
        loop {
            match Spinlock::<28>::acquire() {
                Ok(_) => {
                    Self::address(addr as u16, false);
                    Self::writehal(bytes, false)?;
                    return Self::readhal(buffer)
                },
                _ => continue,
            }
        }
    }
}



impl Write<SevenBitAddress> for I2CMasterInterface<1> {
    type Error = Error;

    fn write(&mut self, addr: SevenBitAddress, bytes: &[u8]) -> Result<(), Self::Error> {
        // Wait for lock.
        loop {
            match Spinlock::<29>::acquire() {
                Ok(_) => {
                    Self::address(addr as u16, true);
                    return Self::writehal(bytes, true)
                },
                _ => continue,
            }
        }
    }
}

impl WriteRead<SevenBitAddress> for I2CMasterInterface<1> {
    type Error = Error;

    fn write_read(&mut self, addr: SevenBitAddress, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        // Wait for lock.
        loop {
            match Spinlock::<29>::acquire() {
                Ok(_) => {
                    Self::address(addr as u16, true);
                    Self::writehal(bytes, false)?;
                    return Self::readhal(buffer)
                },
                _ => continue,
            }
        }
    }
}

impl Write<TenBitAddress> for I2CMasterInterface<1> {
    type Error = Error;

    fn write(&mut self, addr: TenBitAddress, bytes: &[u8]) -> Result<(), Self::Error> {
        // Wait for lock.
        loop {
            match Spinlock::<29>::acquire() {
                Ok(_) => {
                    Self::address(addr as u16, false);
                    return Self::writehal(bytes, true)
                },
                _ => continue,
            }
        }
    }
}

impl WriteRead<TenBitAddress> for I2CMasterInterface<1> {
    type Error = Error;

    fn write_read(&mut self, addr: TenBitAddress, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        // Wait for lock.
        loop {
            match Spinlock::<29>::acquire() {
                Ok(_) => {
                    Self::address(addr as u16, false);
                    Self::writehal(bytes, false)?;
                    return Self::readhal(buffer)
                },
                _ => continue,
            }
        }
    }
}
