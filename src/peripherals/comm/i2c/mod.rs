//! UART Communication peripheral module.


use crate::error::SystemError;
use crate::sys::SystemResource;


pub mod blocking;


mod config;

pub use self::config::I2CConfig;


pub struct I2CInstance<const N: usize>;

impl<const N: usize> SystemResource for I2CInstance<N> {
	fn acquire() -> Result<Self, SystemError> {
		Ok( Self )
	}
}