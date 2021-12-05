//! Error module.
//! Contains all errors possible in the RP2040.


mod adc;
mod comm;
mod cortex;
mod dma;
mod i2c;
mod spi;
mod system;


pub use self::adc::*;
pub use self::comm::*;
pub use self::cortex::CortexError;
pub use self::i2c::*;
pub use self::dma::*;
pub use self::spi::*;
pub use self::system::SystemError;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// A communication error ocurred.
    Comm(CommError),

    /// An I2C error ocurred.
    I2C(I2CError),

    /// A Cortex M0+ architecture error ocurred.
    Cortex(CortexError),

    /// A DMA Error ocurred.
    Dma(DMAError),

    /// A system error ocurred.
    System(SystemError),
}