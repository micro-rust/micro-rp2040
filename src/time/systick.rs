//! System tick peripheral.
//! Simply specifies the Register type to allow for Atomic Hardware accesses.


use crate::raw::SIORegister;
use crate::sys::{ RESOURCES, SystemResource };


use micro::scb;


pub type Systick = scb::Systick<SIORegister<u32>>;


impl SystemResource for Systick {
    const LOCKNUM : usize = 7;
    const LOCKOFF : u8 = 0;

    fn acquire() -> Result<Self, ()> {
        // Get the offset depending on the Core calling.
        let offset = Self::LOCKOFF + crate::sys::coreid() as u8;

        match unsafe { RESOURCES[Self::LOCKNUM] & (1 << offset) } {
            0 => {
                unsafe { RESOURCES[Self::LOCKNUM] |= 1 << offset; }

                Ok( Self::empty() )
            },

            _ => Err(()),
        }
    }
}
