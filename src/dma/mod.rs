//! Direct Memory Access (DMA) abstraction.

#![deny(warnings)]


use crate::raw::AtomicRegister;


pub mod buffer;

mod channel;
mod handle;
mod stream;



#[link_section = ".systemdata.DMAHANDLES"]
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
    fn raw(&self) -> &'static mut [AtomicRegister<u32>; 16];
    
    /// Returns a reference to the asynchronous DMA Handle.
    fn handle<'a>(&self) -> &'a mut DMAHandle;

    /// Launches the DMA Channel.
    fn launch(&self);

    /// Aborts the DMA Channel.
    fn abort(&self);

    /// Stops the DMA Channel.
    fn stop(&self);

    /// Resumes a stopped DMA Channel.
    fn resume(&self);
}
