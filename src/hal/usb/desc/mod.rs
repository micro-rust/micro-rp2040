//! USB descriptors module.


mod configuration;
mod endpoint;
mod interface;
mod string;

pub use self::endpoint::EndpointDescriptor;
pub use self::interface::InterfaceDescriptor;
pub use self::string::StringDescriptor;



/// Common trait for all Descriptors.
pub trait Descriptor {
    type Writer: DescriptorWriter<Descriptor = Self>;
}


/// Common trait for all Descriptor Writers.
pub trait DescriptorWriter {
    type Descriptor;

    /// Creates a new Descriptor Writer.
    fn create(desc: &'static Self::Descriptor) -> Self;

    /// Writes the maximum possible amount of data into the buffer.
    fn bufwrite(&mut self, buf: &mut [u8]) -> Option<usize>;

    /// Resets the writer to reutilize it.
    fn reset(&mut self);
}
