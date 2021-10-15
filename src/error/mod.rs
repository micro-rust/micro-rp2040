//! Error module.
//! Contains all errors possible in the RP2040.


mod comm;
mod cortex;
mod dma;
mod system;



pub use self::comm::*;
pub use self::cortex::CortexError;
pub use self::dma::*;
pub use self::system::SystemError;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// A communication error ocurred.
    Comm(CommError),

    /// A Cortex M0+ architecture error ocurred.
    Cortex(CortexError),

    /// A DMA Error ocurred.
    Dma(DMAError),

    /// A system error ocurred.
    System(SystemError),
}