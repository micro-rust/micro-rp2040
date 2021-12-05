//! Initialization procedure of the BSS regions.
//! Makes use of DMA channels to offload the CPU.

use crate::prelude::*;

extern "C" {
    static mut __ssysbss0 : u32;
    static mut __esysbss0 : u32;

    static mut __ssysbss1 : u32;
    static mut __esysbss1 : u32;

    static mut __sbss : u32;
    static mut __ebss : u32;

    static mut __suninit : u32;
    static mut __euninit : u32;
}



pub(crate) fn bss() {
	// Create a ZERO in a non-stack location for persistence.
	// Use base of SRAM4 + 32 * 4 (middle of the dynamic vector, won't
	// be loaded until user code so this is safe).
    let zeroptr = 0x20003000 as *mut u32;
    unsafe { core::ptr::write_volatile(zeroptr, 0u32) }

    // Build the DMA streams and let them do their work.
    // Initialize the BSS, UNINIT, SYSBSS0 and SYSBSS1 sections.
    
    unsafe {
        let s = &mut __ssysbss0 as *mut u32;
        let e = &mut __esysbss0 as *mut u32;

        if s < e {
            dmazero(0, zeroptr, s, e);
        }

        let s = &mut __ssysbss1 as *mut u32;
        let e = &mut __esysbss1 as *mut u32;

        if s < e {
            dmazero(1, zeroptr, s, e);
        }

        let s = &mut __suninit as *mut u32;
        let e = &mut __euninit as *mut u32;

        if s < e {
            dmazero(2, zeroptr, s, e);
        }

        let s = &mut __sbss as *mut u32;
        let e = &mut __ebss as *mut u32;

        if s < e {
            dmazero(3, zeroptr, s, e);
        }
    }
}


/// Uses the given DMA to zero a memory region.
#[inline(never)]
fn dmazero(n: usize, src: *mut u32, start: *mut u32, end: *mut u32) {
    // Quiet, unpaced, increment write, word size, enable.
    const CTRL: u32 = (1 << 21) | (0x3F << 15) | (1 << 5) | (0x2 << 2) | 1;

    // Get reference to the DMA block.
    let dma = unsafe { &mut *((0x50000000 + (n * 0x40)) as *mut [AtomicRegister<u32>; 4]) };

    // Get transfer count.
    let count = (end as u32 - start as u32) >> 2;

    // Program the DMA block.
    dma[0].write(src as u32);
    dma[1].write(end as u32);
    dma[2].write(count);
    dma[3].write(CTRL);
}
