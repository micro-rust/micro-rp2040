//! Abstraction over a DMA stream.
//! Handles the low level configuration of the stream.


use crate::error::SystemError;
use crate::raw::AtomicRegister;

use micro::Register;
use micro::drivers::Data;

use super::{ DMAChannelTrait, DMAHandle, buffer::* };



pub struct Stream<'a, DMA: DMAChannelTrait> {
    /// DMA Channel used.
    dma: &'a mut DMA,
}



impl<'a, DMA: DMAChannelTrait> Stream<'a, DMA> {
    /// Creates a stream to copy a RAM data buffer into another.
    #[inline(never)]
    pub fn copy<D: Data, SRC: SourceBuffer<'a, D>, DEST: DestinationBuffer<'a, D>>(dma: &'a mut DMA, source: SRC, target: DEST, options: Option<(bool, bool)>) -> Result<Self, SystemError> {
        const CFG: u32 = (0x3F << 15) | (1 << 5) | (1 << 4) | 1;

        // Get sizes.
        let size = (source.size() as u32, target.size() as u32);

        // Size safety checks.
        #[cfg(not(feature = "skip-size-safety-checks"))]
        {
            if size.0 != size.1 { return Err( SystemError::UnequalBufferSize ) }
        }

        // Get addresses.
        let addr = (source.addr(), target.addr());

        // Configure the DMA.
        configure(dma.raw(), CFG | ((D::SIZE as u32 >> 1) << 2), addr, size, options)?;

        // Enable the core's DMA IRQ.
        match crate::sys::coreid() {
            0 => {
                dma.irq0enable();
                dma.irq1disable();
            },

            _ => {
                dma.irq1enable();
                dma.irq0disable();
            },
        }

        // Mark the handle as used.
        dma.handle().reset();

        Ok( Self { dma, } )
    }

    /// Aborts the DMA Stream, if it is still ongoing and has not errored.
    pub fn abort(&mut self) -> Result<(), ()> {
        if self.dma.handle().abortable() {
            self.dma.handle().abort();
            self.dma.abort();

            return Ok(());
        }

        Err(())
    }

    /// Launches the Stream and returns the asynchronous handle of the Stream.
    #[inline]
    pub fn launch(&mut self) -> &'a mut DMAHandle {
        self.dma.launch();
        self.dma.handle().launch();
        self.dma.handle()
    }

    /// Resumes the DMA Stream, if it is still paused/stopped and has not errored.
    pub fn resume(&mut self) -> Result<(), ()> {
        if self.dma.handle().resumable() {
            self.dma.handle().resume();
            self.dma.resume();

            return Ok(());
        }

        Err(())
    }

    /// Stops the DMA Stream, if it is still ongoing and has not errored.
    pub fn stop(&mut self) -> Result<(), ()> {
        if self.dma.handle().stoppable() {
            self.dma.stop();
            self.dma.handle().stop();

            return Ok(());
        }

        Err(())
    }
}



#[inline(never)]
fn configure(dma: &mut [AtomicRegister<u32>; 16], cfg: u32, addr: (u32, u32), size: (u32, u32), options: Option<(bool, bool)>) -> Result<(), SystemError> {
    // Address overlap safety checks.
    #[cfg(not(feature = "skip-address-safety-checks"))]
    overlap(addr, size)?;

    // Configure the source address.
    dma[0].write(addr.0);

    // Configure the destination address.
    dma[1].write(addr.1);

    // Configure the number of transfers.
    dma[2].write(size.0);

    // Modify configuration with the options.
    let cfg = match options {
        Some(opts) => match opts {
            (true,   true) => cfg | (1 << 22) | (1 << 1),
            (true,  false) => cfg | (1 << 22),
            (false,  true) => cfg | (1 <<  1),
            (false, false) => cfg,
        },

        _ => cfg,
    };

    // Configure the control register.
    dma[4].write(cfg);

    Ok(())
}


#[inline(never)]
fn overlap(addr: (u32, u32), size: (u32, u32)) -> Result<(), SystemError> {
    // Check that the buffers are not the same.
    if addr.0 == addr.1 { return Err( SystemError::BufferOverlap ) }

    // Get the end of each buffer.
    let end = (addr.0 + size.0, addr.1 + size.1);

    if (addr.0 < addr.1) && (end.0 > addr.1) { return Err( SystemError::BufferOverlap ) }
    if (addr.1 < addr.0) && (end.1 > addr.0) { return Err( SystemError::BufferOverlap ) }

    Ok(())
}
