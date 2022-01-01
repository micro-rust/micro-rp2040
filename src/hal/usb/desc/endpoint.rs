//! USB Endpoint descriptor.


use super::{ Descriptor, DescriptorWriter };


#[repr(packed)]
pub struct EndpointDescriptor<'a> {
    /// Length of this descriptor (7 bytes).
    bLength: u8,

    /// Type of the descriptor.
    bDescriptorType: u8,

    /// Endpoint address.
    bEndpointAddress: u8,

    /// Attributes.
    bmAttributes: u8,

    /// Maximum packet size.
    wMaxPacketSize: u16,

    /// Poll interval.
    bInterval: u8,

    /// Optional reference to the next Endpoint.
    next: Option<&'a EndpointDescriptor<'a>>,
}


impl<'a> EndpointDescriptor<'a> {
    /// Creates an empty Isochronous Endpoint descriptor.
    pub const fn isochronous(bEndpointAddress: u8, sync: u8, usage: u8, size: u16) -> Self {
        let wMaxPacketSize = match size {
            0 => 1,
            1024.. => 1023,
            _ => size
        };

        EndpointDescriptor {
            bLength: 7,
            bDescriptorType: 0x05,
            bEndpointAddress,
            bmAttributes: ((sync & 0x3) << 2) | ((usage & 0x3) << 4) | 0x1,
            wMaxPacketSize,
            bInterval: 1,

            next: None,
        }
    }

    /// Creates an Interrupt Endpoint descriptor.
    pub const fn interrupt(bEndpointAddress: u8, interval: u8, size: u16) -> Self {
        let wMaxPacketSize = match size {
            0 => 1,
            64.. => 64,
            _ => size
        };


        EndpointDescriptor {
            bLength: 7,
            bDescriptorType: 0x05,
            bEndpointAddress,
            bmAttributes: 0x3,
            wMaxPacketSize,
            bInterval: 1,

            next: None,
        }
    }

    /// Creates a Bulk Endpoint descriptor.
    pub const fn bulk(bEndpointAddress: u8, size: u16) -> Self {
        let wMaxPacketSize = match size {
            0 => 1,
            64.. => 64,
            _ => size
        };


        EndpointDescriptor {
            bLength: 7,
            bDescriptorType: 0x05,
            bEndpointAddress,
            bmAttributes: 0x2,
            wMaxPacketSize,
            bInterval: 0,

            next: None,
        }
    }

    /// Creates a Control Endpoint descriptor.
    pub const fn control(bEndpointAddress: u8, size: u16) -> Self {
        let wMaxPacketSize = match size {
            0 => 1,
            64.. => 64,
            _ => size
        };


        EndpointDescriptor {
            bLength: 7,
            bDescriptorType: 0x05,
            bEndpointAddress,
            bmAttributes: 0x0,
            wMaxPacketSize,
            bInterval: 0,

            next: None,
        }
    }

    /// Chains the given Endpoint descriptor to this descriptor.
    pub const fn chain(mut self, next: &'a Self) -> Self {
        self.next = Some(next);
        self
    }
}


impl<'a> Descriptor for EndpointDescriptor<'a> {
    type Writer = EndpointDescriptorWriter<'a>;
}


pub struct EndpointDescriptorWriter<'a> {
    /// Reference to the Endpoint Descriptor.
    desc: &'a EndpointDescriptor<'a>,

    /// Counter of how many bytes have been sent.
    sent: usize,
}

impl<'a> EndpointDescriptorWriter<'a> {
    /// Sends the next low 4 byte.
    fn byte(&mut self) -> Option<u8> {
        self.sent += 1;

        match self.sent {
            1 => Some( self.desc.bLength ),
            2 => Some( self.desc.bDescriptorType ),
            3 => Some( self.desc.bEndpointAddress ),
            4 => Some( self.desc.bmAttributes ),
            5 => Some( ((self.desc.wMaxPacketSize >> 0) & 0xFF) as u8 ),
            6 => Some( ((self.desc.wMaxPacketSize >> 8) & 0xFF) as u8 ),
            7 => Some( self.desc.bInterval ),

            _ => {
                self.sent = 7;
                None
            },
        }
    }
}


impl<'a> DescriptorWriter for EndpointDescriptorWriter<'a> {
    type Descriptor = EndpointDescriptor<'a>;

    fn create(desc: &'static Self::Descriptor) -> Self {
        Self {
            desc,
            sent: 0,
        }
    }

    fn bufwrite(&mut self, buffer: &mut [u8]) -> Option<usize> {
        // Get count of the number of bytes.
        let mut i = 0;

        for byte in buffer {
            match self.byte() {
                Some(b) => *byte = b,
                _ => match i {
                    0 => return None,
                    _ => return Some(i),
                },
            }

            i += 1;
        }

        Some(i)
    }

    fn reset(&mut self) {
        self.sent = 0;
    }
}
