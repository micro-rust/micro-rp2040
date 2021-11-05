//! Asynchronous handle for UART communications.


use core::future::Future;
use core::pin::Pin;
use core::task::{ Context, Poll };



#[repr(transparent)]
pub struct UartHandle(pub(crate) u32);



impl UartHandle {
    /// Used flag.
    /// This flag indicates that the DMA Channel is in use.
    const USED: u32 = 0x000F;

    /// Launch/Busy flag.
    /// This flag is set when the DMA Channel has started a stream.
    const BUSY: u32 = 0x00F0;

    /// Done/Complete flag.
    /// This flag is set when the DMA Channel has completed the stream.
    const DONE: u32 = 0xF000;

    /// Error flag.
    /// This flag indicates that an error ocurred during the stream.
    const ERROR: u32 = 0x800000;

    /// Overrun error flag.
    /// This flag indicates that an overrun error ocurred.
    const OERROR: u32 = 0x400000;

    /// Break error flag.
    /// This flag indicates that an break error ocurred.
    const BERROR: u32 = 0x100000;

    /// Parity error flag.
    /// This flag indicates that an parity error ocurred.
    const PERROR: u32 = 0x080000;

    /// Framing error flag.
    /// This flag indicates that an framing error ocurred.
    const FERROR: u32 = 0x040000;

    /// Receive timeout error flag.
    /// This flag indicates that an receive timeout error ocurred.
    const RTERROR: u32 = 0x020000;

    /// Transmit timeout error flag.
    /// This flag indicates that transmit timeout error ocurred.
    const TTERROR: u32 = 0x010000;

    /// Static initializer.
    #[inline(always)]
    pub const fn new() -> UartHandle {
        UartHandle(0)
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

    /// Parses a Masked Interrupt Status register to update the info after an IRQ.
    #[inline(always)]
    pub fn errors(&mut self, mis: u32) {
        match mis {
            0 => (),
            _ => self.0 |= ((mis >> 4) | 0x80) << 16,
        }
    }
}



impl super::super::CommHandle for UartHandle {
    fn finished(&self) -> bool {
        //(self.0 & (Self::DONE | Self::ERROR)) != 0
        true
    }
}



impl Future for UartHandle {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        use super::super::CommHandle;

        if self.finished() {
            return Poll::Ready(());
        }

        Poll::Pending
    }
}