//! Configuration Descriptor module.


use super::{ Descriptor, interface::InterfaceDescriptor };



#[derive(Clone, Copy)]
pub struct ConfigurationDescriptor<'a> {
    /// Length of this descriptor.
    bLength: u8,

    /// Descriptor type.
    bDescriptorType: u8,

    /// Total length in bytes.
    wTotalLength: u16,

    /// Number of interfaces in this configuration.
    bNumInterfaces: u8,

    /// Number of interfaces in this configuration.
    bConfigurationValue: u8,

    /// Index of String Descriptor describing this configuration.
    iConfiguration: u8,

    /// Attributes of this configuration.
    bmAttributes: u8,

    /// Maximum Power Consumption in 2mA units.
    bMaxPower: u8,

    /// Pointer to the first Interface of this Configuration.
    interface: &'a InterfaceDescriptor<'a>,

    /// Optional reference to the next Configuration.
    next: Option<&'a ConfigurationDescriptor<'a>>,
}


impl<'a> ConfigurationDescriptor<'a> {
    /// Static initialzier.
    pub const fn new(interface: &'a InterfaceDescriptor, iConfiguration: u8, bConfigurationValue: u8) -> Self {
        Self {
            bLength: 9,
            bDescriptorType: 0x02,
            wTotalLength: interface.addsize(9),
            bNumInterfaces: interface.count(0),
            bConfigurationValue,
            iConfiguration,
            bmAttributes: 0,
            bMaxPower: 250,
            interface,
            next: None,
        }
    }

    /// Chains a Configuration Descriptor into this descriptor.
    pub const fn chain(mut self, next: &'a Self) -> Self {
        self.next = Some(next);
        self
    }
}

impl<'a> Descriptor for ConfigurationDescriptor<'a> {
    type Writer = ConfigurationDescriptorWriter<'a>;
}