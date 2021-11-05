//! System handlers for DMA interrupts.


use crate::peripherals::dma::DMAHANDLES;
use crate::raw::AtomicRegister;
use crate::sys::ints::InterruptSystem;

use micro::Register;


/// Handler for DMA IRQ 0.
/// Checks the state of all DMA channels asserting this IRQ and processes them.
#[inline(never)]
pub(crate) fn dma0() {
    // Clear interrupt in NVIC.
    InterruptSystem::clearpending(11);

    // Get the Interrupt Status register.
    let status = unsafe { &mut *(0x5000040C as *mut AtomicRegister<u32>) };

    // Read which channels are asserting the IRQ.
    let channels = status.read();

    // Clear the interrupts.
    status.write( channels );

    // Check all channels.
    (0..=12).for_each(|idx| match (channels >> idx) & 1 {
        1 => {
            // Get the Channel's control register.
            let ctrl = unsafe { &mut *((0x5000000C + (0x40 * idx)) as *mut AtomicRegister<u32>) }.read();

            // Update the handle.
            unsafe { DMAHANDLES[idx].update(ctrl) };
        },

        0 => (),

        _ => unreachable!(),
    });
}




/// Handler for DMA IRQ 1.
/// Checks the state of all DMA channels asserting this IRQ and processes them.
#[inline(never)]
pub(crate) fn dma1() {
    // Clear interrupt in NVIC.
    InterruptSystem::clearpending(12);

    // Get the Interrupt Status register.
    let status = unsafe { &mut *(0x5000041C as *mut AtomicRegister<u32>) };

    // Read which channels are asserting the IRQ.
    let channels = status.read();

    // Clear the interrupts.
    status.write( channels );

    // Check all channels.
    (0..=12).for_each(|idx| match (channels >> idx) & 1 {
        1 => {
            // Get the Channel's control register.
            let ctrl = unsafe { &mut *((0x5000000C + (0x40 * idx)) as *mut AtomicRegister<u32>) }.read();

            // Update the handle.
            unsafe { DMAHANDLES[idx].update(ctrl) };
        },

        0 => (),

        _ => unreachable!(),
    });
}
