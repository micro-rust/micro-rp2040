//! Container for an optimized DMA Channel.


use crate::raw::AtomicRegister;

use micro::Register;
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
        unsafe { super::DMAHANDLES[N] }
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
}