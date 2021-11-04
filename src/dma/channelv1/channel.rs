//! DMA Channel.
//! Abstraction over one of the 12 DMA Channels in the RP2040.


use crate::dma::DMAHandle;
use crate::error::*;
use crate::raw::AtomicRegister;
use crate::sync::Syslock;
use crate::sys::{ RESOURCES, SystemResource };


use micro::Register;


pub struct DMAChannel<const N: usize>;

impl<const N: usize> DMAChannel<N> {
    const ADDR: usize = 0x50000000 + { N * 0x40 };
}


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


impl<const N: usize> super::DMAChannelTrait for DMAChannel<N> {
    fn raw(&self) -> &'static mut [AtomicRegister<u32>; 16] {
        unsafe { &mut *(Self::ADDR as *mut _) }
    }

    fn handle(&self) -> &'static mut DMAHandle {
        unsafe { &mut crate::dma::asynchronous::DMAHANDLES[N as usize] }
    }

    fn launch(&self) {
        let dma: &'static mut [AtomicRegister<u32>; 16] = unsafe { &mut *(Self::ADDR as *mut _) };

        dma[3].write( dma[3].read() | 1);
    }

    fn abort(&self) {
        let abort: &'static mut AtomicRegister<u32> = unsafe { &mut *(0x50000444 as *mut _) };

        // Send abort signal.
        abort.set(1 << N);

        // Wait all transactions are flushed.
        while abort.read() != 0 { micro::asm::nop() }
    }

    fn stop(&self) {
        let dma: &'static mut [AtomicRegister<u32>; 16] = unsafe { &mut *(Self::ADDR as *mut _) };

        dma[3].write( dma[3].read() & !1);
    }

    fn resume(&self) {
        let dma: &'static mut [AtomicRegister<u32>; 16] = unsafe { &mut *(Self::ADDR as *mut _) };

        dma[3].write( dma[3].read() | 1);
    }

    fn irq0(&self, f: Option<fn()>) {
        unsafe { crate::dma::asynchronous::DMACALLBACK0[N] = f }
    }

    fn irq1(&self, f: Option<fn()>) {
        unsafe { crate::dma::asynchronous::DMACALLBACK1[N] = f }
    }
}
