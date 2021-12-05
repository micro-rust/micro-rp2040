//! Initialization procedure of the vector table.


use crate::prelude::*;


extern "C" {
    static __svt0: u32;
}

#[inline(always)]
pub(crate) fn vectortable() {
    // Load the vectortable and modify VTOR.
    unsafe { crate::sys::init::load(
        0x20040000 as *mut u32,
        0x20040040 as *mut u32,

        & __svt0 as *const u32,
    )};

    // Reference to the Interrupt table.
    let table = unsafe { &mut *(0x20040040 as *mut [SIORegister<u32>; 32]) };

    // Reference to the ISER register.
    let iser = unsafe { &mut *(0xE000E100 as *mut SIORegister<u32>) };

    // Configure DMA0 interrupt.
    table[11].write( crate::sys::ints::handlers::dma0 as u32 );

    // Enable all necessary interrupts.
    iser.write(1 << 11);
}