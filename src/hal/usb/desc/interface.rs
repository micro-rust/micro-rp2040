//! USB Interface descriptor.


use super::{ Descriptor, DescriptorWriter };


#[repr(packed)]
pub struct InterfaceDescriptor<'a> {
    /// Length of this descriptor (9 bytes).
    bLength: u8,

    /// Type of the descriptor.
    bDescriptorType: u8,

    /// Number of the interface.
    bInterfaceNumber: u8,

    /// Value used to select alternative setting.
    bAlternateSetting: u8,

    /// Number of endpoints for this interface.
    bNumEndpoints: u8,

    /// Class code.
    bInterfaceClass: u8,

    /// Subclass code.
    bInterfaceSubClass: u8,

    /// Protocol code.
    bInterfaceProtocol: u8,

    /// Index of string descriptor describing this interface.
    iInterface: u8,

    /// Optional reference to the next interface.
    next: Option<&'a InterfaceDescriptor<'a>>,
}


impl<'a> InterfaceDescriptor<'a> {
    /// Creates an empty descri9ptor.
    #[inline(always)]
    pub const fn new() -> Self {
        InterfaceDescriptor {
            bLength: core::mem::size_of::<Self>() as u8,
            bDescriptorType: 0x04,
            bInterfaceNumber: 0x00,
            bAlternateSetting: 0x00,
            bNumEndpoints: 0x00,
            bInterfaceClass: 0x00,
            bInterfaceSubClass: 0x00,
            bInterfaceProtocol: 0x00,
            iInterface: 0x00,

            next: None,
        }
    }

    /// Sets the interface number.
    pub const fn inum(mut self, n: u8) -> Self {
        self.bInterfaceNumber = n;
        self
    }

    /// Sets the number of endpoints.
    pub const fn numep(mut self, n: u8) -> Self {
        self.bNumEndpoints = n;
        self
    }

    /// Sets the alternate setting.
    pub const fn altsetting(mut self, n: u8) -> Self {
        self.bAlternateSetting = n;
        self
    }

    /// Sets the interface class and subclass.
    pub const fn class(mut self, c: u8, s: u8) -> Self {
        self.bInterfaceClass = c;
        self.bInterfaceSubClass = s;
        self
    }

    /// Sets the interface protocol.
    pub const fn protocol(mut self, n: u8) -> Self {
        self.bInterfaceProtocol = n;
        self
    }

    /// Sets the index of the string descriptor.
    pub const fn stridx(mut self, i: u8) -> Self {
        self.iInterface = i;
        self
    }

    /// Counts the number of interfaces linked.
    pub const fn count(&self, mut count: u8) -> u8 {
        count += 1;

        match self.next {
            Some(i) => i.count(count),
            _ => count
        }
    }
}
