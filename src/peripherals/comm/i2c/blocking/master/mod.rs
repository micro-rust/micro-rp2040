//! Blocking master interface of an I2C instance.
//! TODO : Implement resource acquisition and dropping.

use crate::prelude::*;
use crate::{
    peripherals::pins::i2c::*, sys::clocks::Clocks, math::UInt32,
};

use super::super::{ I2CInstance, I2CConfig };


mod eh;
mod interface;
//mod micro;


pub use self::interface::I2CMasterInterface;



pub struct I2CMaster<const N: usize, SDA: I2CSdaPin<N>, SCL: I2CSclPin<N>> {
    /// SDA Pin.
    sda: SDA,

    /// SCL Pin.
    scl: SCL,
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

        // Configure    Fast     TX Control  Slave OFF  Restart   Master ON
        i2c[0].write((0x2 << 1) | (1 << 8) | (1 << 6) | (1 << 5) | (1 << 0));


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
        i2c[ 7].write( u32::from(  hcnt  ) );
        i2c[ 8].write( u32::from(  lcnt  ) );
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

    /// Creates a new concurrent I2C interface.
    #[inline]
    pub fn interface(&mut self) -> I2CMasterInterface<N> {
        I2CMasterInterface::create()
    }
}


impl<const N: usize, SDA: I2CSdaPin<N>, SCL: I2CSclPin<N>> Drop for I2CMaster<N, SDA,SCL> {
    fn drop(&mut self) {
        // Release the resource.

        // Reset the peripheral.

    }
}
