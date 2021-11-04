//! DMA Channel module.


use crate::dma::DMAHandle;
use crate::raw::AtomicRegister;


mod any;
mod channel;


pub use self::any::DMAChannelDyn;
pub use self::channel::DMAChannel;


/// Trait for all DMA Channel abstractions.
pub trait DMAChannelTrait: Sized {

	/// Returns the raw DMA Channel block.
	fn raw(&self) -> &'static mut [AtomicRegister<u32>; 16];
	
	/// Returns a reference to the asynchronous DMA Handle.
	fn handle(&self) -> &'static mut DMAHandle;

	/// Launches the DMA Channel.
	fn launch(&self);

	/// Aborts the DMA Channel.
	fn abort(&self);

	/// Stops the DMA Channel.
	fn stop(&self);

	/// Resumes a stopped DMA Channel.
	fn resume(&self);

	/// Sets the IRQ of this channel for Core 0.
	fn irq0(&self, f: Option<fn()>);

	/// Sets the IRQ of this channel for Core 1.
	fn irq1(&self, f: Option<fn()>);
}
