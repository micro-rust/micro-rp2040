//! SPI peripheral module.


use crate::prelude::*;


mod common;
mod interface;
mod master;


pub use self::common::*;
pub use self::interface::*;
pub use self::master::SPIMaster;


#[link_section = ".systemdata.SPIREFCNT"]
pub(self) static mut SPIREFCNT: [u16; 2] = [0u16; 2];



pub struct SPIInstance<const N: usize> where [(); 2+N]: Sized;

impl<const N: usize> SystemResource for SPIInstance<N> where [(); 2+N]: Sized {
    fn acquire() -> Result<Self, SystemError> {
        match Resources::spi::<N>() {
            Some(_) => Ok( Self ),
            _ => Err( SystemError::PeripheralNotAvailable ),
        }
    }

    fn release(&mut self) {
        DropResources::spi::<N>();

        core::mem::forget(self);
    }
}


impl<const N: usize> Drop for SPIInstance<N> where [(); 2+N]: Sized {
    fn drop(&mut self) {
        DropResources::spi::<N>();
    }
}
