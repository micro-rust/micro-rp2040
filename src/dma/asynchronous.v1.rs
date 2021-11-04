//! Asynchronous control of DMA asbtractions.


use crate::error::DMAError;
use crate::raw::AtomicRegister;


use micro::Register;



#[link_section = ".systemdata.DMAHANDLES"]
pub(super) static mut DMAHANDLES : [Option<DMAHandle>; 12] = [None; 12];


#[link_section = ".systemdata.DMAIRQ0"]
pub(super) static mut DMACALLBACK0 : [Option<fn()>; 12] = [None; 12];

#[link_section = ".systemdata.DMAIRQ1"]
pub(super) static mut DMACALLBACK1 : [Option<fn()>; 12] = [None; 12];


/// A DMA handle provides a safe asynchronous abstraction over an ongoing DMA 
/// stream or transaction.
/// WARNING! : This handle cannot be droped without completion or cancellation.
pub struct DMAHandle(pub(super) u32);

impl DMAHandle {
    const ENABLED: u32 = 0xF0000000;

    const COMPLETE: u32 = 0x0000000F;

    const CANCELLED: u32 = 0x00F00000;

    const AHB_ERROR: u32 = 0x000F0000;

    const READ_ERROR: u32 = 0x00000F00;

    const WRITE_ERROR: u32 = 0x0000F000;


    /// Creates a new enabled DMA handle.
    #[inline(always)]
    pub const fn new() -> Self {
        DMAHandle(0xF0000000)
    }

    /// Returns `true` if this is completed.
    #[inline(always)]
    pub fn poll(&self) -> bool {
        (self.0 & Self::COMPLETE) == Self::COMPLETE
    }

    /// Blocks the thread until completion.
    /// This function can block indefinitively.
    #[inline]
    pub fn join(&self) {
        while (self.0 & Self::COMPLETE) == 0 { micro::asm::nop() }
    }

    /// Returns `true` if an error ocurred with this handle.
    pub fn errored(&self) -> bool {
        (self.0 & (Self::READ_ERROR | Self::WRITE_ERROR)) != 0
    }

    /// If this handle errored, returns the error that ocurred.
    pub fn error(&self) -> Option<DMAError> {
        if (self.0 & Self::READ_ERROR) == Self::READ_ERROR {
            Some( DMAError::ReadError )
        } else if (self.0 & Self::WRITE_ERROR) == Self::WRITE_ERROR {
            Some( DMAError::WriteError )
        } else if (self.0 & Self::AHB_ERROR) == Self::AHB_ERROR {
            Some( DMAError::AHBError )
        } else {
            None
        }
    }

    /// Returns `true` if the DMA transaction was cancelled.
    pub fn cancelled(&self) -> bool {
        (self.0 & Self::CANCELLED) == Self::CANCELLED
    }

    /// Crate-private method to set the complete flag.
    pub(self) fn set_complete(&mut self) {
        self.0 |= Self::COMPLETE
    }

    /// Crate-private method to set the AHB error flag.
    pub(self) fn set_ahb(&mut self) {
        self.0 |= Self::AHB_ERROR
    }

    /// Crate-private method to set the read error flag.
    pub(self) fn set_read(&mut self) {
        self.0 |= Self::READ_ERROR
    }

    /// Crate-private method to set the read error flag.
    pub(self) fn set_write(&mut self) {
        self.0 |= Self::WRITE_ERROR
    }
}


impl Drop for DMAHandle {
    fn drop(&mut self) {
        if !(self.cancelled() || self.poll()) {
            panic!()
        }
    }
}




/// Interrupt routine for DMA interrupts in DMA IRQ 0.
/// Handles asynchronous DMA transactions' state machines and standard
/// function callbacks.
#[inline(never)]
pub(crate) fn dmairq0() {
    // Read which DMA channels are causing an IRQ.
    let status : &'static mut AtomicRegister<u32> = unsafe { &mut *(0x5000040C as *mut _) };
    let mut channels = status.read() << 1;

    // Check which DMA IRQ is raised.
    for i in 0..12 {
        match (channels >> 1) & 1 {
            // Channel did not trigger.
            0 => continue,

            // Channel did trigger.
            _ => {
                // Check if its handle is set.
                match unsafe { DMAHANDLES[i] } {
                    // Update handle.
                    Some(ref handle) => {
                        // Reference to the register block.
                        let block : &'static mut [AtomicRegister<u32>; 16] = unsafe { &mut *((0x50000000 + (0x40 * i)) as *mut _) };

                        // Read in CTRL register.
                        let ctrl = block[4].read();

                        // Check if an error ocurred.
                        if (ctrl >> 31) == 1 {
                            // Check if it's a read error.
                            if (ctrl & (1 << 30)) != 0 {
                                handle.set_read();
                            }

                            // Check if it's a write error.
                            if (ctrl & (1 << 30)) != 0 {
                                handle.set_write();
                            }

                            handle.set_ahb();
                        }

                        // Check if the busy flag is down.
                        if (ctrl & (1 << 24)) == 0 {
                            handle.set_complete();
                        }
                    },

                    _ => ()
                }


                // Check if the callback is set.
                match unsafe { DMACALLBACK0[i] } {
                    // Jump to callback.
                    Some(irq) => irq(),

                    _ => ()
                }
            },
        }
    }

    // Clear all channels IRQ pending.
    status.set(channels);
}




/// Interrupt routine for DMA interrupts in DMA IRQ 0.
/// Handles asynchronous DMA transactions' state machines and standard
/// function callbacks.
#[inline(never)]
pub(crate) fn dmairq1() {
    // Read which DMA channels are causing an IRQ.
    let status : &'static mut AtomicRegister<u32> = unsafe { &mut *(0x5000041C as *mut _) };
    let mut channels = status.read() << 1;

    // Check which DMA IRQ is raised.
    for i in 0..12 {
        match (channels >> 1) & 1 {
            // Channel did not trigger.
            0 => continue,

            // Channel did trigger.
            _ => {
                // Check if its handle is set.
                match unsafe { DMAHANDLES[i] } {
                    // Update handle.
                    Some(ref handle) => {
                        // Reference to the register block.
                        let block : &'static mut [AtomicRegister<u32>; 16] = unsafe { &mut *((0x50000000 + (0x40 * i)) as *mut _) };

                        // Read in CTRL register.
                        let ctrl = block[4].read();

                        // Check if an error ocurred.
                        if (ctrl >> 31) == 1 {
                            // Check if it's a read error.
                            if (ctrl & (1 << 30)) != 0 {
                                handle.set_read();
                            }

                            // Check if it's a write error.
                            if (ctrl & (1 << 30)) != 0 {
                                handle.set_write();
                            }

                            handle.set_ahb();
                        }

                        // Check if the busy flag is down.
                        if (ctrl & (1 << 24)) == 0 {
                            handle.set_complete();
                        }
                    },

                    _ => ()
                }


                // Check if the callback is set.
                match unsafe { DMACALLBACK1[i] } {
                    // Jump to callback.
                    Some(irq) => irq(),

                    _ => ()
                }
            },
        }
    }

    // Clear all channels IRQ pending.
    status.set(channels);
}
