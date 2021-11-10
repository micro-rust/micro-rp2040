//! Blocking I2C interfaces.


/// Master interface.
mod master;

// Slave interface.
//mod slave;


pub use self::master::I2CMaster;