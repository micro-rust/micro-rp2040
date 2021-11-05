//! System tick peripheral.
//! Simply specifies the Register type to allow for Atomic Hardware accesses.


use crate::error::SystemError;
use crate::raw::SIORegister;
use crate::sys::SystemResource;


use micro::scb;


pub type Systick = scb::Systick<SIORegister<u32>>;


impl SystemResource for Systick {
    fn acquire() -> Result<Self, SystemError> {
        use crate::sys::RESOURCES;

        // Get the offset depending on the Core calling.
        let offset = 16 + match crate::sys::coreid() { 0 => 0, _ => 1, };

        match unsafe { RESOURCES[0] & (1 << offset) } {
            0 => {
                unsafe { RESOURCES[0] |= 1 << offset; }

                Ok( Self::empty() )
            },

            _ => Err( SystemError::PeripheralNotAvailable ),
        }
    }
}
