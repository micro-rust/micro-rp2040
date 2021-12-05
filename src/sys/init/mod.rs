//! Common functionality for Core 0 and Core 1 initialization.


#![deny(warnings)]


use crate::prelude::*;
use micro::asm::*;


pub(super) mod core0;
pub(super) mod core1;



/// Non DMA way to load a memory region.
#[inline(never)]
pub(self) unsafe fn load(mut s: *mut u32, e: *mut u32, mut l: *const u32) {
    use core::ptr::{
        read_volatile as read,
        write_volatile as write,
    };

    if s >= e { return; }
    //if s >= e { micro::asm::bkpt::<255>(); }

    while s < e {
        write(s, read(l));
        s = s.offset(1);
        l = l.offset(1);
    }
}

/// Non DMA way to zero out a memory region.
#[inline(never)]
pub(self) unsafe fn zero(mut s: *mut u32, e: *mut u32) {
    use core::ptr::write_volatile as write;

    if s >= e { return; }

    while s < e {
        write(s, 0u32);
        s = s.offset(1);
    }
}


/// Loads a vectortable and reloads the VTOR.
#[inline(never)]
pub(self) unsafe fn vectortable(s: *mut u32, e: *mut u32, l: *const u32) {
    // Load the vectortable into the bottom of the stack.
    load(s, e, l);

    // Reference to the VTOR register.
    let vtor = &mut *(0xE000ED08 as *mut SIORegister<u32>);

    // Disable interrupts.
    micro::asm::cpsid_i();

    // Write relocation address.
    vtor.write(0x20040000);

    // Set all memory barriers.
    isb();
    dmb();
    dsb();

    // Restore interrupts.
    micro::asm::cpsie_i();
}

#[inline(never)]
pub(self) fn mpu(vect: u32, stack: u32) {
    // Set all memory barriers.
    isb();
    dmb();
    dsb();

    // Reference to the MPU RBAR and RASR registers.
    let rbar = unsafe { &mut *(0xE000ED9C as *mut SIORegister<u32>) };
    let rasr = unsafe { &mut *(0xE000EDA0 as *mut SIORegister<u32>) };

    // REGION 0.
    // ************************************************************************
    // Region 0 address.
    const R0ADDR: u32 = 0x00000000 | (1 << 4) | 0;

    // Region 0 configuration.
    //                     RW RW            SCB        2^32 Bytes
    const R0CFG: u32 = (0b011 << 24) | (0b111 << 16) | (31 << 1) | 1;


    rbar.write(R0ADDR);
    rasr.write(R0CFG);
    // ************************************************************************



    // REGION 7.
    // ************************************************************************
    // Region 7 address.
    const R7ADDR: u32 = (1 << 4) | 7;

    // Region 7 configuration.
    //                     RO RO            SCB            192 / 256      2^8 Bytes
    const R7CFG: u32 = (0b111 << 24) | (0b111 << 16) | (0b11000000 << 8) | (7 << 1) | 1;


    rbar.write(R7ADDR | vect);
    rasr.write(R7CFG);
    // ************************************************************************

    // REGION 5.
    // ************************************************************************
    // Region 5 address.
    const R5ADDR: u32 = (1 << 4) | 5;

    // Region 5 configuration.
    //                   None None          SCB       2^12 Bytes
    const R5CFG: u32 = (0b000 << 24) | (0b111 << 16) | (11 << 1) | 1;


    rbar.write(R5ADDR | stack);
    rasr.write(R5CFG);
    // ************************************************************************


    // REGION 6.
    // ************************************************************************
    // Region 6 address.
    const R6ADDR: u32 = (1 << 4) | 6;

    // Region 6 configuration.
    //                     RW RW            SCB           768 / 1024      2^10 Bytes
    const R6CFG: u32 = (0b011 << 24) | (0b111 << 16) | (0b00000011 << 8) | (9 << 1) | 1;


    rbar.write(R6ADDR | (stack + 0xC00));
    rasr.write(R6CFG);
    // ************************************************************************

    // Regions 1, 2, 3 and 4 -> Available for the user.

    // Reference to the MPU Control register.
    let ctrl = unsafe { &mut *(0xE000ED94 as *mut SIORegister<u32>) };

    // Enable the MPU.
    ctrl.write(1);


    // Set all memory barriers.
    isb();
    dmb();
    dsb();
}
