//! Implementation of a blocking SPI Master interface.


use core::marker::PhantomData;

use crate::error::*;
use crate::prelude::*;
use crate::hal::pins::spi::*;
use crate::hal::spi::{ SPIMaster, SPIConfig, bitrate };

use micro::drivers::Data;



pub struct SPIMasterInterface<const N: usize, D: Data, CS: SPICsMasterPin> {
    /// Compacted configuration registers.
    pub cfg: u32,

    /// Chip Select pin.
    cs: CS,

    _d: PhantomData<D>,
}

impl<const N: usize, D: Data, CS: SPICsMasterPin> SPIMasterInterface<N, D, CS> {
    /// Creates a new interface from the given SPI Master.
    pub fn create<SCLK: SPISckPin<N>, MOSI: SPIMosiPin<N>, MISO: SPIMisoPin<N>>(_: &mut SPIMaster<N, SCLK, MOSI, MISO>, cs: CS, config: SPIConfig, bits: u8) -> Result<(Self, u32), SPIError> {
        // Build the configuration.
        let (prescale, postdiv, baud) = bitrate(config.baud);

        // Create the configuration.
        let cfg = (prescale << 16) | (postdiv << 8) | config.regs | ((bits as u32 - 1) & 0xF);

        // Configure the CS pin.
        cs.config();

        Ok( (
            Self {
                cfg,
                cs,
                _d: PhantomData,
            },
            baud
        ))

    }
}


impl<D: Data, CS: SPICsMasterPin> SPIMasterInterface<0, D, CS> {
    /// Address of the SPI0 peripheral.
    const ADDRESS: usize = 0x4003C000;

    /// Acquires internal arbitration of the SPI peripheral and sends data through the SPI bus.
    pub fn write(&mut self, bytes: &[u8]) -> Result<(), ()> {
        // SPI peripheral register block.
        let spi = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 10]) };

        loop {
            match Spinlock::<26>::acquire() {
                Ok(_) => {
                    configure(spi, self.cfg);
                    self.cs.select();
                    let result = writehalu8( spi, bytes );
                    self.cs.deselect();

                    return result;
                },
                _ => continue,
            }
        }
    }

    /// Acquires internal arbitration of the SPI peripheral and sends and receives data through the SPI bus.
    pub fn write_read(&mut self, bytes: &[u8], buffer: &mut [u8]) -> Result<(), ()> {
        // SPI peripheral register block.
        let spi = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 10]) };

        loop {
            match Spinlock::<26>::acquire() {
                Ok(_) => {
                    configure(spi, self.cfg);
                    self.cs.select();
                    let result = writereadhalu8( spi, bytes, buffer );
                    self.cs.deselect();

                    return result;
                },
                _ => continue,
            }
        }
    }
}

impl<D: Data, CS: SPICsMasterPin> SPIMasterInterface<1, D, CS> {
    /// Address of the SPI0 peripheral.
    const ADDRESS: usize = 0x40040000;

    /// Acquires internal arbitration of the SPI peripheral and sends data through the SPI bus.
    pub fn write(&mut self, bytes: &[u8]) -> Result<(), ()> {
        // SPI peripheral register block.
        let spi = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 10]) };

        loop {
            match Spinlock::<27>::acquire() {
                Ok(_) => {
                    configure(spi, self.cfg);
                    self.cs.select();
                    let result = writehalu8( spi, bytes );
                    self.cs.deselect();

                    return result;
                },
                _ => continue,
            }
        }
    }

    /// Acquires internal arbitration of the SPI peripheral and sends and receives data through the SPI bus.
    pub fn write_read(&mut self, bytes: &[u8], buffer: &mut [u8]) -> Result<(), ()> {
        // SPI peripheral register block.
        let spi = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 10]) };

        loop {
            match Spinlock::<27>::acquire() {
                Ok(_) => {
                    configure(spi, self.cfg);
                    self.cs.select();
                    let result = writereadhalu8( spi, bytes, buffer );
                    self.cs.deselect();

                    return result;
                },
                _ => continue,
            }
        }
    }
}

fn configure(spi: &mut [AtomicRegister<u32>; 10], cfg: u32) {
    // Disable the SPI to configure it.
    spi[1].write(0);

    // Configure the SPI prescaler and postscaler.
    spi[4].write(cfg >> 16);
    spi[0].write(cfg);

    // Enable the SPI.
    spi[1].write(1 << 1);
    //spi[1].write((1 << 1) | 1);
}


fn writehalu8(spi: &mut [AtomicRegister<u32>; 10], bytes: &[u8]) -> Result<(), ()> {
    // Send the bytes.
    for byte in bytes {
        // Wait until SPI TX not full.
        while (spi[3].read() & (1 << 1)) == 0 {}

        // Send the byte.
        spi[2].write( *byte as u32 );
    }

    Ok(())
}

fn writereadhalu8(spi: &mut [AtomicRegister<u32>; 10], bytes: &[u8], buffer: &mut [u8]) -> Result<(), ()> {

    // Get mutable indices.
    let mut txidx = 0;
    let mut rxidx = 0;

    // Get the end of the arrays.
    let txend = bytes.len();
    let rxend = bytes.len();

    // Status variable.
    let mut status;

    // Loop until all data has been received / sent.
    loop {
        // Get status.
        status = spi[3].read();

        // If there are TX bytes left and TX FIFO is not full, send.
        if (txidx < txend) && ((status & (1 << 1)) != 0) {
            spi[2].write(bytes[txidx] as u32);
            txidx += 1;
        }

        // If there are RX bytes left and RX FIFO is not empty, read.
        if (rxidx < rxend) && ((status & (1 << 2)) != 0) {
            buffer[rxidx] = spi[2].read() as u8;
            rxidx += 1;
        }
    }
}
