//! Container for an optimized DMA Channel.


use crate::prelude::*;

use micro::asm::nop;

use super::DMAChannelTrait;
use super::DMAHandle;


/// Compile time evaluated DMA Channel.
pub struct DMAChannel<const N: usize>;



impl<const N: usize> DMAChannel<N> {
    const ADDR: usize = 0x50000000 + { N * 0x40 };
}



impl<const N: usize> DMAChannelTrait for DMAChannel<N> {
    fn raw(&mut self) -> &'static mut [AtomicRegister<u32>; 16] {
        unsafe { &mut *(Self::ADDR as *mut _) }
    }

    fn handle<'a>(&mut self) -> &'a mut DMAHandle {
        unsafe { &mut super::DMAHANDLES[N] }
    }

    fn launch(&mut self) {
        let trigger = unsafe { &mut *(0x50000430 as *mut AtomicRegister<u32>) };
        trigger.set(1 << N);
    }

    fn abort(&mut self) {
        let abort = unsafe { &mut *(0x50000444 as *mut AtomicRegister<u32>) };
        abort.set(1 << N);

        while abort.read() != 0 { nop() }
    }

    fn stop(&mut self) {
        let ctrl = unsafe { &mut *((Self::ADDR + 0x00C) as *mut AtomicRegister<u32>) };
        ctrl.clear(1);
    }

    fn resume(&mut self) {
        let ctrl = unsafe { &mut *((Self::ADDR + 0x00C) as *mut AtomicRegister<u32>) };
        ctrl.set(1);
    }

    #[inline(always)]
    fn irq0enable(&mut self) {
        let inte = unsafe { &mut *(0x50000404 as *mut AtomicRegister<u32>) };
        inte.set(1 << N);
    }

    #[inline(always)]
    fn irq1enable(&mut self) {
        let inte = unsafe { &mut *(0x50000414 as *mut AtomicRegister<u32>) };
        inte.set(1 << N);
    }

    #[inline(always)]
    fn irq0disable(&mut self) {
        let inte = unsafe { &mut *(0x50000404 as *mut AtomicRegister<u32>) };
        inte.clear(1 << N);
    }

    #[inline(always)]
    fn irq1disable(&mut self) {
        let inte = unsafe { &mut *(0x50000414 as *mut AtomicRegister<u32>) };
        inte.clear(1 << N);
    }
}



impl<const N: usize> SystemResource for DMAChannel<N> {
    fn acquire() -> Result<Self, SystemError> {
        match Syslock::acquire() {
            Ok(_) => match Resources::dma::<N>() {
                Some(_) => Ok( Self ),

                _ => Err( SystemError::PeripheralNotAvailable ),
            },

            _ => Err( SystemError::NoSystemLock ),
        }
    }

    fn release(&mut self) {
        DropResources::dma::<N>();

        core::mem::forget(self);
    }
}



impl<const N: usize> Drop for DMAChannel<N> {
    fn drop(&mut self) {
        // Zero out the registers.
        let dma = unsafe { &mut *(Self::ADDR as *mut [AtomicRegister<u32>; 4]) };

        dma[0].write(0);
        dma[1].write(0);
        dma[2].write(0);
        dma[3].write(0);

        DropResources::dma::<N>();
    }
}