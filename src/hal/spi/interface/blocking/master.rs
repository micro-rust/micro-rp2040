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

        // Configura the CS pin.
        cs.config();

        Ok((
            Self { cfg, cs, _d: PhantomData },
            baud
        ))
    }
}

impl<CS: SPICsMasterPin> SPIMasterInterface<0, u8, CS> {
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

    /// Acquires internal arbitration of the SPI peripheral and sends data through the SPI bus.
    /// Does not assert the CS pin. Used to clear the bus.
    pub fn write_deasserted(&mut self, bytes: &[u8]) -> Result<(), ()> {
        // SPI peripheral register block.
        let spi = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 10]) };

        loop {
            match Spinlock::<26>::acquire() {
                Ok(_) => {
                    configure(spi, self.cfg);
                    let result = writehalu8( spi, bytes );

                    return result;
                },
                _ => continue,
            }
        }
    }

    /// Acquires internal arbitration of the SPI peripheral and reads data through the SPI bus.
    pub fn read(&mut self, buffer: &mut [u8], noop: Option<u8>) -> Result<(), ()> {
        // SPI peripheral register block.
        let spi = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 10]) };

        loop {
            match Spinlock::<26>::acquire() {
                Ok(_) => {
                    configure(spi, self.cfg);
                    self.cs.select();
                    let result = readhalu8( spi, buffer, noop );
                    self.cs.deselect();

                    return result;
                },
                _ => continue,
            }
        }
    }

    /// Acquires internal arbitration of the SPI peripheral and sends and receives data through the SPI bus.
    pub fn write_read(&mut self, bytes: &[u8], buffer: &mut [u8], noop: Option<u8>) -> Result<(), ()> {
        // SPI peripheral register block.
        let spi = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 10]) };

        loop {
            match Spinlock::<26>::acquire() {
                Ok(_) => {
                    configure(spi, self.cfg);
                    self.cs.select();
                    let result = writereadhalu8( spi, bytes, buffer, noop );
                    self.cs.deselect();

                    return result;
                },
                _ => continue,
            }
        }
    }
}

impl<D: Data, CS: SPICsMasterPin> SPIMasterInterface<1, D, CS> {
    /// Address of the SPI1 peripheral.
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

    /// Acquires internal arbitration of the SPI peripheral and reads data through the SPI bus.
    pub fn read(&mut self, buffer: &mut [u8], noop: Option<u8>) -> Result<(), ()> {
        // SPI peripheral register block.
        let spi = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 10]) };

        loop {
            match Spinlock::<27>::acquire() {
                Ok(_) => {
                    configure(spi, self.cfg);
                    self.cs.select();
                    let result = readhalu8( spi, buffer, noop );
                    self.cs.deselect();

                    return result;
                },
                _ => continue,
            }
        }
    }

    /// Acquires internal arbitration of the SPI peripheral and sends and receives data through the SPI bus.
    pub fn write_read(&mut self, bytes: &[u8], buffer: &mut [u8], noop: Option<u8>) -> Result<(), ()> {
        // SPI peripheral register block.
        let spi = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 10]) };

        loop {
            match Spinlock::<27>::acquire() {
                Ok(_) => {
                    configure(spi, self.cfg);
                    self.cs.select();
                    let result = writereadhalu8( spi, bytes, buffer, noop );
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


/// Writes to TX FIFO, ignores RX and cleans up after it.
#[inline(never)]
fn writehalu8(spi: &mut [AtomicRegister<u32>; 10], bytes: &[u8]) -> Result<(), ()> {
    // Loop over all the bytes.
    for byte in bytes {
        // Wait until the FIFO is writable.
        while (spi[3].read() & (1 << 1)) != 0 { micro::asm::nop() }

        // Write the data.
        spi[2].write( *byte as u32 );
    }

    // Drain RX FIFO.
    drainrx(spi);

    Ok(())
}

/// Reads from RX FIFO while writing the given NO OP value.
/// If no NO OP value was given, defaults to 0x00.
#[inline(never)]
fn readhalu8(spi: &mut [AtomicRegister<u32>; 10], buffer: &mut [u8], noop: Option<u8>) -> Result<(), ()> {
    // Get default value.
    let default = match noop {
        Some(x) => x as u32,
        _ => 0x00,
    };

    // Send a NO OP byte to get the transaction going.
    spi[2].write( default );

    // Loop over all the buffer.
    for byte in buffer {
        // Send a NO OP byte.
        spi[2].write( default );

        // Wait until the FIFO is writable.
        while (spi[3].read() & (1 << 2)) != 0 { micro::asm::nop() }

        // Read the data.
        *byte = spi[2].read() as u8;
    }

    // Drain RX FIFO.
    drainrx(spi);

    Ok(())
}


/// Writes to TX FIFO while reads from RX FIFO.
/// If RX is bigger than TX, uses the NO OP value given.
/// If no NO OP value was given, it uses 0x00 as default.
#[inline(never)]
fn writereadhalu8(spi: &mut [AtomicRegister<u32>; 10], bytes: &[u8], buffer: &mut [u8], noop: Option<u8>) -> Result<(), ()> {
    // Get default value.
    let default = match noop {
        Some(x) => x as u32,
        _ => 0x00,
    };

    // Get mutable indices.
    let mut txidx = 0;
    let mut rxidx = 0;

    // Reserve mutable status.
    let mut status;

    // Get the end of the arrays.
    let txend = bytes.len();
    let rxend = bytes.len();


    while (rxidx < rxend) && (txidx < txend) {
        // Get updated status.
        status = spi[3].read();

        // If there is space in the TX FIFO.
        if (status & (1 << 1)) != 0 {
            // If there is data to be sent, send it.
            // If there is data to be received, send NO OP value.
            if txidx < txend {
                spi[2].write( bytes[txidx] as u32 );
                txidx += 1;
            } else if rxidx < rxend {
                spi[2].write( default );
            }
        }


        // If there is data in the RX FIFO.
        if ((status & (1 << 2)) != 0) && (rxidx < rxend) {
            buffer[rxidx] = spi[2].read() as u8;
            rxidx += 1;
        }
    }

    // Drain RX FIFO.
    drainrx(spi);

    Ok(())
}


/// Drains the RX FIFO of an SPI peripheral.
#[inline(never)]
fn drainrx(spi: &mut [AtomicRegister<u32>; 10]) {
    // Drain RX FIFO.
    while (spi[3].read() & (1 << 2)) != 0 { spi[2].read(); }

    // Wait for shifting to finish.
    while (spi[3].read() & (1 << 4)) != 0 { micro::asm::nop() }

    // Drain RX FIFO again.
    while (spi[2].read() & (1 << 2)) != 0 { spi[2].read(); }

    // Remove Overrun flag.
    spi[8].write(1);
}
