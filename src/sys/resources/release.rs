//! Controls the release of resources.


use crate::prelude::*;

use super::RESOURCES;



pub(crate) struct DropResources;


impl DropResources {
    /// General function to release resources.
    #[inline(always)]
    unsafe fn release<const R: usize, const O: usize>() {
    	loop {
    		match Syslock::acquire() {
    			Ok(_) => {
                    RESOURCES[R] &= !(1 << O);
                    return;
                },
    			_ => continue,
    		}
    	}
    }

    /// Releases the Systick instance.
    pub fn systick() {
        match crate::sys::coreid() {
            0 => unsafe { Self::release::<0, 16>() },
            _ => unsafe { Self::release::<0, 17>() },
        }
    }

    /// Releases the DMA Channel`N` instance.
    pub fn dma<const N: usize>() {
        unsafe { Self::release::<1, N>() }
    }

    /// Releases release the I2C`N` instance.
    pub fn i2c<const N: usize>() {
        unsafe { Self::release::<2, { N }>() }
    }

    /// Releases release the I2C0 instance.
    pub fn i2c0() {
        unsafe { Self::release::<2, 0>() }
    }

    /// Releases release the I2C1 instance.
    pub fn i2c1() {
        unsafe { Self::release::<2, 1>() }
    }

    /// Releases release the SPI`N` instance.
    pub fn spi<const N: usize>() where [(); 2+N]: Sized {
        unsafe { Self::release::<2, { 2+N }>() }
    }

    /// Releases release the SPI0 instance.
    pub fn spi0() {
        unsafe { Self::release::<2, 2>() }
    }

    /// Releases the SPI1 instance.
    pub fn spi1() {
        unsafe { Self::release::<2, 3>() }
    }

    /// Releases the UART`N` instance.
    pub fn uart<const N: usize>() where [(); 4+N]: Sized {
        unsafe { Self::release::<2, { 4+N }>() }
    }

    /// Releases the UART0 instance.
    pub fn uart0() {
        unsafe { Self::release::<2, 4>() }
    }

    /// Releases the UART1 instance.
    pub fn uart1() {
        unsafe { Self::release::<2, 5>() }
    }

    /// Releases the ADC instance.
    pub fn adc() {
        unsafe { Self::release::<2, 16>() }
    }

    /// Releases the Gpio`N` instance.
    pub fn pin<const N: usize>() {
        unsafe { Self::release::<3, N>() }
    }
}
