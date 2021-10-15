//! DMA Channel.
//! Abstraction over one of the 12 DMA Channels in the RP2040.


use crate::error::*;
use crate::raw::AtomicRegister;
use crate::sync::Syslock;
use crate::sys::{ RESOURCES, SystemResource };

use micro::Register;



pub struct DMAChannel<const N: usize>;


impl<const N: usize> SystemResource for DMAChannel<N> {
    const LOCKNUM: usize = 1;
    const LOCKOFF: u8 = N as u8;

    fn acquire() -> Result<Self, Error> {
        match Syslock::acquire() {
            Some(_) => match unsafe { RESOURCES[Self::LOCKNUM] & (1 << Self::LOCKOFF) } {
                0 => unsafe {
                    RESOURCES[Self::LOCKNUM] |= 1 << Self::LOCKOFF;

                    Ok( Self )
                },

                _ => Err( Error::System( SystemError::DMAChannelNotAvailable ) ),
            },

            _ => Err( Error::System( SystemError::NoSystemLock ) ),
        }
    }
}


impl<const N: usize> DMAChannel<N> {
    /// Address of the Channel's block.
    const BLOCK : u32 = 0x50000000 + (0x10 * N as u32);
}



/// Inner function to avoid excessive code duplication.
#[inline(never)]
fn irqset(n: usize, intreg: &'static mut AtomicRegister<u32> ,irq: fn(), quiet: bool) {
    // Clear the IRQ mask to not trigger while switching.
    intreg.clear(1 << n);

    // Set the function, it cannot trigger yet.
    unsafe { crate::dma::asynchronous::DMACALLBACK0[n] = Some(irq); }

    // Check if QUIET IRQ is needed.
    let ctrl : &'static mut [AtomicRegister<u32>; 16] = unsafe { &mut *((0x50000000 + (0x10 * n)) as *mut _) };

    if quiet { ctrl[4].set(1 << n); }
    else { ctrl[4].clear(1 << n); }

    // Enable the trigger.
    intreg.set(1 << n);
}

