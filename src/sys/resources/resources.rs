//! Controls the acquisition of resources.



use super::RESOURCES;



pub(crate) struct Resources;


impl Resources {
    /// General function to acquire resources.
    #[inline(always)]
    unsafe fn acquire<const R: usize, const O: usize>() -> Option<()> {
        match (RESOURCES[R] >> O) & 1 {
            0 => {
                RESOURCES[R] |= 1 << O;

                Some(())
            },

            _ => None,
        }
    }

    /// Attempts to acquire the Systick instance.
    /// This function assumes that the system lock has been acquired.
    pub fn systick() -> Option<()> {
        match crate::sys::coreid() {
            0 => unsafe { Self::acquire::<0, 16>() },
            _ => unsafe { Self::acquire::<0, 17>() },
        }
    }

    /// Attempts to acquire the DMA Channel`N` instance.
    /// This function assumes that the system lock has been acquired.
    pub fn dma<const N: usize>() -> Option<()> {
        unsafe { Self::acquire::<1, N>() }
    }

    /// Attempts to acquire the I2C`N` instance.
    /// This function assumes that the system lock has been acquired.
    pub fn i2c<const N: usize>() -> Option<()> {
        unsafe { Self::acquire::<2, { N }>() }
    }

    /// Attempts to acquire the I2C0 instance.
    /// This function assumes that the system lock has been acquired.
    pub fn i2c0() -> Option<()> {
        unsafe { Self::acquire::<2, 0>() }
    }

    /// Attempts to acquire the I2C1 instance.
    /// This function assumes that the system lock has been acquired.
    pub fn i2c1() -> Option<()> {
        unsafe { Self::acquire::<2, 1>() }
    }

    /// Attempts to acquire the SPI`N` instance.
    /// This function assumes that the system lock has been acquired.
    pub fn spi<const N: usize>() -> Option<()> where [(); 2+N]: Sized {
        unsafe { Self::acquire::<2, { 2+N }>() }
    }

    /// Attempts to acquire the SPI0 instance.
    /// This function assumes that the system lock has been acquired.
    pub fn spi0() -> Option<()> {
        unsafe { Self::acquire::<2, 2>() }
    }

    /// Attempts to acquire the SPI1 instance.
    /// This function assumes that the system lock has been acquired.
    pub fn spi1() -> Option<()> {
        unsafe { Self::acquire::<2, 3>() }
    }

    /// Attempts to acquire the UART`N` instance.
    /// This function assumes that the system lock has been acquired.
    pub fn uart<const N: usize>() -> Option<()> where [(); 4+N]: Sized {
        unsafe { Self::acquire::<2, { 4+N }>() }
    }

    /// Attempts to acquire the UART0 instance.
    /// This function assumes that the system lock has been acquired.
    pub fn uart0() -> Option<()> {
        unsafe { Self::acquire::<2, 4>() }
    }

    /// Attempts to acquire the UART1 instance.
    /// This function assumes that the system lock has been acquired.
    pub fn uart1() -> Option<()> {
        unsafe { Self::acquire::<2, 5>() }
    }

    /// Attempts to acquire the ADC instance.
    /// This function assumes that the system lock has been acquired.
    pub fn adc() -> Option<()> {
        unsafe { Self::acquire::<2, 16>() }
    }

    /// Attempts to acquire the Gpio`N` instance.
    /// This function assumes that the system lock has been acquired.
    pub fn pin<const N: usize>() -> Option<()> {
        unsafe { Self::acquire::<3, N>() }
    }
}
