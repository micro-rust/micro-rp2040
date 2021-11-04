//! Any DMA Channel.
//! Access to any DMA Channel that is available.
//! This is a dynamic, more heavy abstraction over the DMA Channels
//! that allows more flexibility.


use crate::dma::DMAHandle;
use crate::error::SystemError;
use crate::raw::AtomicRegister;
use crate::sync::Syslock;
use crate::sys::RESOURCES;


use micro::Register;


#[repr(C)]
pub struct DMAChannelDyn {
    /// Channel number.
    chn: u16,

    /// Channel offset from base register.
    off: u16,
}

impl DMAChannelDyn {
    const LOCKNUM: usize = 1;

    /// Acquire the given DMA Channel.
    /// WARNING: This method will fail if the requested Channel is in use.
    pub fn channel() -> Result<Self, SystemError> {
        match Syslock::acquire() {
            Some(_) => {
                let lock: u32 = unsafe { RESOURCES[Self::LOCKNUM] };

                for i in 0..12 {
                    match lock & (1 << i) {
                        0 => {
                            unsafe { RESOURCES[Self::LOCKNUM] |= 1 << i; }

                            return Ok( DMAChannelDyn { chn: i, off: 0x40 * i } );
                        },

                        _ => continue,
                    }
                }

                Err( SystemError::DMAChannelNotAvailable )
            },

            _ => Err( SystemError::NoSystemLock ),
        }
    }
}


impl super::DMAChannelTrait for DMAChannelDyn {
    fn raw(&self) -> &'static mut [AtomicRegister<u32>; 16] {
        unsafe { &mut *((0x50000000 + self.off as usize) as *mut _) }
    }

    fn handle(&self) -> &'static mut DMAHandle {
        unsafe { &mut crate::dma::asynchronous::DMAHANDLES[self.chn as usize] }
    }

    fn launch(&self) {
        let dma: &'static mut [AtomicRegister<u32>; 16] = unsafe { &mut *((0x50000000 + self.off as usize) as *mut _) };

        dma[3].write( dma[3].read() | 1);
    }

    fn abort(&self) {
        let abort: &'static mut AtomicRegister<u32> = unsafe { &mut *(0x50000444 as *mut _) };

        // Send abort signal.
        abort.set(1 << self.chn);

        // Wait all transactions are flushed.
        while abort.read() != 0 { micro::asm::nop() }
    }

    fn stop(&self) {
        let dma: &'static mut [AtomicRegister<u32>; 16] = unsafe { &mut *((0x50000000 + self.off as usize) as *mut _) };

        dma[3].write( dma[3].read() & !1);
    }

    fn resume(&self) {
        let dma: &'static mut [AtomicRegister<u32>; 16] = unsafe { &mut *((0x50000000 + self.off as usize) as *mut _) };

        dma[3].write( dma[3].read() | 1);
    }

    fn irq0(&self, f: Option<fn()>) {
        unsafe { crate::dma::asynchronous::DMACALLBACK0[self.chn as usize] = f }
    }

    fn irq1(&self, f: Option<fn()>) {
        unsafe { crate::dma::asynchronous::DMACALLBACK1[self.chn as usize] = f }
    }
}
