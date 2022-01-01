//! Basic GPIO abstraction.
//! All other abstractions build on top of this.


use crate::prelude::*;

use super::GPIOPin;


/// Zero sized type of a GPIO.
pub struct Gpio<const N: u8>;


impl<const N: u8> Gpio<N> {
    /// System Resource ID.
    pub(crate) const fn id() -> SystemResource {
        match N {
            0 => SystemResource::Gpio0,
            1 => SystemResource::Gpio1,
            2 => SystemResource::Gpio2,
            3 => SystemResource::Gpio3,
            4 => SystemResource::Gpio4,
            5 => SystemResource::Gpio5,
            6 => SystemResource::Gpio6,
            7 => SystemResource::Gpio7,
            8 => SystemResource::Gpio8,
            9 => SystemResource::Gpio9,

            10 => SystemResource::Gpio10,
            11 => SystemResource::Gpio11,
            12 => SystemResource::Gpio12,
            13 => SystemResource::Gpio13,
            14 => SystemResource::Gpio14,
            15 => SystemResource::Gpio15,
            16 => SystemResource::Gpio16,
            17 => SystemResource::Gpio17,
            18 => SystemResource::Gpio18,
            19 => SystemResource::Gpio19,

            20 => SystemResource::Gpio20,
            21 => SystemResource::Gpio21,
            22 => SystemResource::Gpio22,
            23 => SystemResource::Gpio23,
            24 => SystemResource::Gpio24,
            25 => SystemResource::Gpio25,
            26 => SystemResource::Gpio26,
            27 => SystemResource::Gpio27,
            28 => SystemResource::Gpio28,
            29 => SystemResource::Gpio29,

            _ => panic!(),
        }
    }
}

impl<const N: u8> GPIOPin for Gpio<N> {
    const IO  : u32 = 0x40014000 + {0x08 * (N as u32)};
    const PAD : u32 = 0x4001C000 + {0x04 * (N as u32)} + 0x04;

    fn intclear(&mut self) {
        // Offset from PROC0_INTE0 register.
        let r: usize = 4 * (N as usize / 8);

        // Offset inside the INTEx register for the start of the control bits.
        let o: usize = 4 * (N as usize % 8);

        super::intclear(r, o)
    }

    fn reset(&mut self) {
        // Clear all interrupts.
        {
            let inte = unsafe { &mut *(0x40014100 as *mut [AtomicRegister<u32>; 36]) };

            let r: usize = 4 * (N as usize >> 3);
            let o: usize = 4 * (N as usize & 0x7);

            inte[r +  0].clear(0xF << o);
            inte[r + 12].clear(0xF << o);
            inte[r + 24].clear(0xF << o);
        }

        // Configure PAD register.
        {
            let pad = unsafe { &mut *(Self::PAD as *mut AtomicRegister<u32>) };

            pad.write(0x8C);
        }

        // Configure IO register.
        {
            let io = unsafe { &mut *((Self::IO + 4) as *mut AtomicRegister<u32>) };

            io.write( (0x2 << 12) | 0x1F);
        }
    }
}

impl<const N: u8> Acquire for Gpio<N> {
    fn acquire() -> Result<Self, SystemError> {
        match Syslock::acquire() {
            Ok(_) => {
                unsafe { AcquireSystem::acquire(Self::id())? };
                Ok(Self)
            },

            Err(e) => Err(e),
        }
    }
}

impl<const N: u8> Release for Gpio<N> {
    fn release(&mut self) -> Result<(), SystemError> {
        loop {
            match Syslock::acquire() {
                Ok(_) => {
                    unsafe { ReleaseSystem::release(Self::id())? };
                    core::mem::forget(self);
                    return Ok(())
                },
                _ => continue,
            }
        }
    }
}


impl<const N: u8> Drop for Gpio<N> {
    fn drop(&mut self) {
        loop {
            match Syslock::acquire() {
                Ok(_) => {
                    unsafe { ReleaseSystem::force(Self::id()) };
                    break;
                },
                _ => continue,
            }
        }
    }
}
