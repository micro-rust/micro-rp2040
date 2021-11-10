//! Configuration of the I2C Master instances.

/// Default DMA configuration for I2C config register.
/// Quiet, unpaced, 
pub(super) const DMACFG: u32 = (1 << 21) | (0x3F << 15) | (0x2 << 2) | 1;

/// Default DMA configuration for I2C0 data read.
/// Quiet, paced by I2Cx RX, WRADDR increment.
pub(super) const DMARD0: u32 = (1 << 21) | (32 << 15) | (1 << 5) | 1;

/// Default DMA configuration for I2C0 data write.
/// Quiet, paced by I2Cx RX, WRADDR increment.
pub(super) const DMAWR0: u32 = (1 << 21) | (33 << 15) | (1 << 4) | 1;

/// Default DMA configuration for I2C1 data read.
/// Quiet, paced by I2Cx RX, WRADDR increment.
pub(super) const DMARD1: u32 = (1 << 21) | (34 << 15) | (1 << 5) | 1;

/// Default DMA configuration for I2C1 data write.
/// Quiet, paced by I2Cx RX, WRADDR increment.
pub(super) const DMAWR1: u32 = (1 << 21) | (35 << 15) | (1 << 4) | 1;
