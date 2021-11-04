//! Asynchronous DMA Stream handle.


use core::future::Future;
use core::pin::Pin;
use core::task::{ Context, Poll };


use micro::asm::nop;


#[repr(C)]
pub struct DMAHandle(u32);


impl DMAHandle {
    /// Used flag.
    /// This flag indicates that the DMA Channel is in use.
    const USED: u32 = 0x000F;

    /// Launch/Busy flag.
    /// This flag is set when the DMA Channel has started a stream.
    const BUSY: u32 = 0x00F0;

    /// Stop flag.
    /// This flag is set when the DMA Channel has stopped but can be resumed.
    const STOP: u32 = 0x0300;

    /// Abort flag.
    /// This flag is set when the DMA Channel has been aborted. It cannot be resumed.
    const ABORT: u32 = 0x0C00;

    /// Done/Complete flag.
    /// This flag is set when the DMA Channel has completed the stream.
    const DONE: u32 = 0xF000;

    /// Error flag.
    /// This flag indicates that an error ocurred during the stream.
    const ERROR: u32 = 0x40000;

    /// Read error flag.
    /// This flag indicates that an error ocurred during the stream.
    const RDERROR: u32 = 0x20000;

    /// Write error flag.
    /// This flag indicates that an error ocurred during the stream.
    const WRERROR: u32 = 0x10000;

    /// Static initializer.
    #[inline(always)]
    pub const fn new() -> Self {
        Self(0)
    }

    /// Returns `true` if the Stream was aborted. 
    #[inline(always)]
    pub fn aborted(&self) -> bool {
        (self.0 & Self::ABORT) != 0
    }

    /// Returs `true` if the Stream is busy.
    #[inline(always)]
    pub fn busy(&self) -> bool {
        (self.0 & Self::BUSY) != 0
    }

    /// Returns `true` if the Stream has completed.
    #[inline(always)]
    pub fn completed(&self) -> bool {
        (self.0 & Self::DONE) != 0
    }

    /// Returns `true` if the Stream has completed.
    #[inline(always)]
    pub fn done(&self) -> bool {
        (self.0 & Self::DONE) != 0
    }

    /// Returs `true` if the Stream is stopped.
    #[inline(always)]
    pub fn stopped(&self) -> bool {
        (self.0 & Self::STOP) != 0
    }

    /// Returs `true` if the Stream has either aborted, completed or errored.
    #[inline(always)]
    pub fn finished(&self) -> bool {
        (self.0 & (Self::ABORT | Self::DONE | Self::ERROR)) != 0
    }

    /// Blocks until the Stream has either aborted, completed or errored.
    #[inline(always)]
    pub fn join(&self) {
        while (self.0 & (Self::ABORT | Self::DONE | Self::ERROR)) == 0 { nop() }
    }

    /// Sets the aborted flag, clears the BUSY flag.
    #[inline(always)]
    pub(crate) fn abort(&mut self) {
        self.0 = (self.0 & !Self::BUSY) | Self::ABORT
    }

    /// Returns `true` if the Stream is can be aborted.
    #[inline(always)]
    pub(crate) fn abortable(&self) -> bool {
        ((self.0 & (Self::BUSY | Self::STOP)) != 0) & ((self.0 & (Self::ABORT | Self::DONE | Self::ERROR)) == 0)
    }

    /// Sets the BUSY flag, clears the STOP flag.
    #[inline(always)]
    pub(crate) fn resume(&mut self) {
        self.0 = (self.0 & !Self::STOP) | Self::BUSY
    }

    /// Returns `true` if the Stream is can be resumed.
    #[inline(always)]
    pub(crate) fn resumable(&self) -> bool {
        ((self.0 & Self::STOP) != 0) & ((self.0 & (Self::ABORT | Self::DONE | Self::ERROR | Self::BUSY)) == 0)
    }

    /// Sets the STOP flag, clears the BUSY flag.
    #[inline(always)]
    pub(crate) fn stop(&mut self) {
        self.0 = (self.0 & !Self::BUSY) | Self::STOP
    }

    /// Returns `true` if the Stream is can be stopped.
    #[inline(always)]
    pub(crate) fn stoppable(&self) -> bool {
        ((self.0 & Self::BUSY) != 0) & ((self.0 & (Self::ABORT | Self::DONE | Self::ERROR | Self::STOP)) == 0)
    }

    /// Resets the handle to used.
    #[inline(always)]
    pub(crate) fn reset(&mut self) {
        self.0 = Self::USED
    }

    /// Sets the launch flag, clears the STOP flag.
    #[inline(always)]
    pub(crate) fn launch(&mut self) {
        self.0 |= Self::BUSY
    }

    /// Parses a control register to update the info after an IRQ.
    pub fn update(&mut self, ctrl: u32) {
        // Set the error flags.
        self.0 |= (ctrl >> 13) & 0x7000;

        // Check for the completion flag.
        match (ctrl >> 24) & 1 {
            1 => self.0 = (self.0 & !Self::BUSY) | Self::DONE,
            0 => (),

            _ => unreachable!(),
        }
    }
}


impl Future for DMAHandle {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.finished() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}