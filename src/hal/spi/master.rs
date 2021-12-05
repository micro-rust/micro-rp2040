//! SPI Master instance.
//! Main instance of an SPI peripheral configured for master mode.
//! From this instance, SPI interfaces can be created for different external devices.

use crate::prelude::*;
use crate::hal::pins::spi::*;

use super::{ SPIInstance };

pub struct SPIMaster<const N: usize, SCLK: SPISckPin<N>, MOSI: SPIMosiPin<N>, MISO: SPIMisoPin<N>> {
    /// Clock pin.
    sclk: SCLK,

    /// Master Out Slave In pin.
    mosi: MOSI,

    /// Master In Slave Out pin.
    miso: MISO,
}

impl<const N: usize, SCLK: SPISckPin<N>, MOSI: SPIMosiPin<N>, MISO: SPIMisoPin<N>> SPIMaster<N, SCLK, MOSI, MISO> where [(); 2+N]: Sized {
    const ADDRESS: usize = 0x4003C000 + (0x4000 * N);

    #[allow(unused_variables)]
    pub fn duplex(spi: SPIInstance<N>, sclk: SCLK, mosi: MOSI, miso: MISO) -> Self {
        // SPI peripheral register block.
        let spi = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 10]) };

        // Disable the SPI.
        spi[0].write(0);

        // Configure the pins.
        sclk.config();
        mosi.config();
        miso.config();

        // Erase the acquired SPI instance to avoid double creation.
        core::mem::forget(spi);

        Self { sclk, mosi, miso }
    }
}
