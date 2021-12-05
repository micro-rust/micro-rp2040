//! I2C Communication peripheral module.


use crate::prelude::*;
use micro::drivers::Handle;

pub use self::config::I2CConfig;


//pub mod asynchronous;
pub mod blocking;
mod config;


#[link_section = ".sysbss0.I2CHANDLES"]
pub(self) static mut HANDLES: [Handle; 2] = [Handle::new(); 2];


pub struct I2CInstance<const N: usize>;

impl<const N: usize> SystemResource for I2CInstance<N> {
    fn acquire() -> Result<Self, SystemError> {
        match Syslock::acquire() {
            Ok(_) => match Resources::i2c::<N>() {
                Some(_) => {
                    Ok( Self )
                },

                _ => return Err( SystemError::PeripheralNotAvailable ),
            },

            _ => return Err( SystemError::NoSystemLock ),
        }
    }

    fn release(&mut self) {
        DropResources::i2c::<N>();

        core::mem::forget(self);
    }
}


impl<const N: usize> Drop for I2CInstance<N> {
    fn drop(&mut self) {
        DropResources::i2c::<N>();
    }
}
