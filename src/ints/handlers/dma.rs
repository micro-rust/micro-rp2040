//! System handlers for DMA interrupts.


use crate::raw::AtomicRegister;
use crate::ints::InterruptSystem;

use micro::Register;


/// Handler for DMA IRQ 0.
/// Checks the state of all DMA channels asserting this IRQ and processes them.
#[inline(never)]
pub(crate) fn dma0() {
    use crate::dma::DMAHANDLES;

    // Clear interrupt in NVIC.
    InterruptSystem::clearpending(11);

    // Read which channels are asserting the IRQ.
    let channels = unsafe { &mut *(0x5000040C as *mut AtomicRegister<u32>) }.read();

    // Check all channels.
    (0..=12).for_each(|idx| match (channels >> idx) & 1 {
        1 => {
            // Get the Channel's control register.
            let ctrl = unsafe { &mut *((0x5000000C + (0x40 * idx)) as *mut AtomicRegister<u32>) }.read();

            // Set errors if there were any.
            unsafe { DMAHANDLES[idx].set_errors(ctrl) };

            // Check for completion.
            match (ctrl >> 24) & 1 {
                0 => unsafe { DMAHANDLES[idx].set_complete() },
                _ => ()
            }
        },

        0 => (),

        _ => unreachable!(),
    });

    // Clear the interrupts.
    unsafe { &mut *(0x5000040C as *mut AtomicRegister<u32>) }.clear( channels );
}




/// Handler for DMA IRQ 1.
/// Checks the state of all DMA channels asserting this IRQ and processes them.
#[inline(never)]
pub(crate) fn dma1() {
    use crate::dma::DMAHANDLES;

    // Clear interrupt in NVIC.
    InterruptSystem::clearpending(12);

    // Read which channels are asserting the IRQ.
    let channels = unsafe { &mut *(0x5000041C as *mut AtomicRegister<u32>) }.read();

    // Check all channels.
    (0..=12).for_each(|idx| match (channels >> idx) & 1 {
        1 => {
            // Get the Channel's control register.
            let ctrl = unsafe { &mut *((0x5000000C + (0x40 * idx)) as *mut AtomicRegister<u32>) }.read();

            // Set errors if there were any.
            unsafe { DMAHANDLES[idx].set_errors(ctrl) };

            // Check for completion.
            match (ctrl >> 24) & 1 {
                0 => unsafe { DMAHANDLES[idx].set_complete() },
                _ => ()
            }
        },

        0 => (),

        _ => unreachable!(),
    });

    // Clear the interrupts.
    unsafe { &mut *(0x5000041C as *mut AtomicRegister<u32>) }.clear( channels );
}
