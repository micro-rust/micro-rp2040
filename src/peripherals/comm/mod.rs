//! Communication peripherals abstractions.

#![deny(warnings)]


pub mod uart;



use core::future::Future;
use core::pin::Pin;
use core::task::{ Context, Poll };


use crate::peripherals::dma::{ DMAChannelTrait, DMAHandle, buffer::SourceBuffer };


use micro::Register;
use micro::drivers::Data;



pub struct AsyncTxInterface<T: AsyncTx, C: DMAChannelTrait> {
    /// Communication peripheral.
    comm: T,

    /// DMA Channel.
    dma: C,
}

impl<T: AsyncTx, C: DMAChannelTrait> AsyncTxInterface<T, C> {
    /// Creates a new interface from the given peirpheral and DMA Channel.
    pub fn create(comm: T, mut dma: C) -> Self {
        // Default configuration for the DMA Channel.
        const CFG: u32 = (1 << 4) | 1;

        // Configure the DMA.
        let dmablock = dma.raw();

        dmablock[1].write(comm.port());
        dmablock[4].write(CFG | (comm.dreq() << 15));

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

        // Create the interface.
        Self { comm, dma, }
    }

    /// Sends the given buffer through the peripheral.
    pub fn send<'b, D: Data, SRC: SourceBuffer<'b, D>>(&mut self, src: SRC, byteswap: bool) -> Option<TXHandle<T::Handle>> where T: Sender<D> {
        // Check that the stream is launchable.
        if !self.dma.handle().ready() { return None }

        // Set the read address and count.
        let dmablock = self.dma.raw();

        dmablock[0].write( src.addr() );
        dmablock[2].write( src.size() as u32 );

        if byteswap { dmablock[4].set(   1 << 22 ) }
        else        { dmablock[4].clear( 1 << 22 ) }

        // Launch the stream.
        self.dma.handle().launch();
        self.dma.launch();

        // Return an async handle.
        Some( TXHandle::create( self.dma.handle(), self.comm.handle() ) )
    }
}



pub trait AsyncRx {
    /// Associated asynchrnous handle type.
    type Handle: CommHandle;

    /// Enables reception of data.
    fn enable(&mut self);

    /// Disables reception of data.
    fn disable(&mut self);

    /// Returns the read port.
    fn port(&self) -> u32;

    /// Returns a reference to the asynchronous handle.
    fn handle<'a>(&self) -> &'a mut Self::Handle;
}



pub trait AsyncTx {
    /// Associated asynchrnous handle type.
    type Handle: CommHandle;

    /// Enables transmission of data.
    fn enable(&mut self);

    /// Disables transmission of data.
    fn disable(&mut self);

    /// Returns the write port.
    fn port(&self) -> u32;

    /// Returns a reference to the asynchronous handle.
    fn handle<'a>(&self) -> &'a mut Self::Handle;

    /// DMA Data request ID.
    fn dreq(&self) -> u32;
}



/// Method used to tag a communication peripheral with the type of data it can receive.
pub trait Receiver<D: Data> {
    /// Associated method to accomodate for the given data type.
    fn config(&mut self);
}



/// Method used to tag a communication peripheral with the type of data it can send.
pub trait Sender<D: Data> {
    /// Associated method to accomodate for the given data type.
    fn config(&mut self);
}



pub trait CommHandle: Future {
    fn finished(&self) -> bool;
}



/// Asynchronous handle for TX operations.
pub struct TXHandle<'a, F: CommHandle> {
    /// DMA Handle.
    dma: &'a mut DMAHandle,

    /// Communication handle.
    comm: &'a mut F,
}

impl<'a, F: CommHandle> TXHandle<'a, F> {
    /// Creates a new asynchandle.
    pub fn create(dma: &'a mut DMAHandle, comm: &'a mut F) -> Self {
        Self { dma, comm }
    }

    /// Returns `true` if the handle has finished.
    #[inline(always)]
    pub fn finished(&self) -> bool {
        self.dma.finished() && self.comm.finished()
    }

    /// Blocks the thread until the handle has completed.
    pub fn join(&self) {
        while !self.finished() { micro::asm::nop() }
    }
}

impl<'a, F: CommHandle> Future for TXHandle<'a, F> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        if self.dma.finished() {
            if self.comm.finished() {
                return Poll::Ready(());
            }
        }

        Poll::Pending
    }
}
