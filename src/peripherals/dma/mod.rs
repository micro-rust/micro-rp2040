//! Direct Memory Access (DMA) abstraction.

#![deny(warnings)]


use crate::raw::AtomicRegister;


pub mod buffer;

mod channel;
mod handle;
mod stream;



#[link_section = ".systemdata0.DMAHANDLES"]
pub(crate) static mut DMAHANDLES : [DMAHandle; 12] = [
    DMAHandle::new(), DMAHandle::new(), DMAHandle::new(), DMAHandle::new(),
    DMAHandle::new(), DMAHandle::new(), DMAHandle::new(), DMAHandle::new(),
    DMAHandle::new(), DMAHandle::new(), DMAHandle::new(), DMAHandle::new(),
];



pub use self::handle::DMAHandle;
pub use self::channel::DMAChannel;
pub use self::stream::Stream;



/// Trait for all DMA Channel abstractions.
pub trait DMAChannelTrait: Sized {

    /// Returns the raw DMA Channel block.
    fn raw(&mut self) -> &'static mut [AtomicRegister<u32>; 16];
    
    /// Returns a reference to the asynchronous DMA Handle.
    fn handle<'a>(&mut self) -> &'a mut DMAHandle;

    /// Launches the DMA Channel.
    fn launch(&mut self);

    /// Aborts the DMA Channel.
    fn abort(&mut self);

    /// Stops the DMA Channel.
    fn stop(&mut self);

    /// Resumes a stopped DMA Channel.
    fn resume(&mut self);

    /// Enables the DMA IRQ 0 for this channel.
    fn irq0enable(&mut self);

    /// Enables the DMA IRQ 1 for this channel.
    fn irq1enable(&mut self);

    /// Disables the DMA IRQ 0 for this channel.
    fn irq0disable(&mut self);

    /// Disables the DMA IRQ 1 for this channel.
    fn irq1disable(&mut self);
}
