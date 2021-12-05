//! System resources.



mod release;
mod resources;



pub(crate) use self::release::DropResources;
pub(crate) use self::resources::Resources;



/// Resources of the RP2040.
#[link_section = ".systembss0.RESOURCES"]
pub(self) static mut RESOURCES : [u32; 8] = [0u32; 8];



pub trait SystemResource: Sized + Drop {
    /// Function to acquire the resource.
    fn acquire() -> Result<Self, crate::error::SystemError>;

    /// Function to release the resource.
    fn release(&mut self);
}


