//! I2C Errors.



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2CError {
    /// The I2C master lost arbitration.
    ArbitrationLost,

    /// A master transfer was attempted while not in master mode.
    NotInMasterMode,

    /// A master tried to read in 10 bit mode without sending restart.
    MasterRead10bitNotRestart,

    /// An ACK was received on the start byte.
    AckOnStartByte,

    /// No device ACK a General call.
    NackGeneralCall,

    /// Some data sent was not ACK.
    NackData,

    /// No device ACK the second bit of a 10-bit address.
    NackAddress10bitByte2,

    /// No device ACK the first bit of a 10-bit address.
    NackAddress10bitByte1,

    /// No device ACK a 7-bit address.
    NackAddress7bit,

    Unknown,
}