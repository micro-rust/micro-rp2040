//! String Descriptor.
//! 
//! How to create a String Descriptor.
//! 
//! ```
//! // First create the String.
//! static STRING: &'static str = "DESCRIPTOR";
//! 
//! // Then create the String descriptor.
//! static STRING1 : StringDescriptor = StringDescriptor::create(&STRING, None);
//! 
//! // You can also concatenate the strings.
//! Remember to concatenate towards the string at index 0 (... 2 -> 1 -> 0).
//! static STRING0 : StringDescriptor = StringDescriptor::create(&STRING, Some(&STRING1));
//! ```



use super::super::{ Descriptor, DescriptorWriter };



pub struct StringDescriptor<'a> {
    /// Size of descriptor in bytes.
    pub(super) bLength: u8,

    /// Descriptor Type.
    bDescriptorType: u8,

    /// Reference to the string.
    string: &'a [u8],

    /// Optional next string.
    next: Option<&'a StringDescriptor<'a>>,
}

impl<'a> StringDescriptor<'a> {
    /// Creates a new String Descriptor Zero.
    pub const fn create(string: &'a [u8], next: Option<&'a StringDescriptor>) -> Self {
        Self {
            bLength: string.len() as u8 + 2,
            bDescriptorType: 0x03,
            string,
            next,
        }
    }
}

impl<'a> Descriptor for StringDescriptor<'a> {
    type Writer = StringDescriptorWriter<'a>;
}





/// Byte Writer for the String Descriptor Zero.
pub struct StringDescriptorWriter<'a> {
    /// Reference to the String Descriptor Zero.
    desc: &'a StringDescriptor<'a>,

    /// Counter of how many bytes of the string have been sent.
    sent: usize,

    /// Flag to check if the header has been written.
    head: bool,
}

impl<'a> DescriptorWriter for StringDescriptorWriter<'a> {
    type Descriptor = StringDescriptor<'a>;

    fn create(desc: &'static Self::Descriptor) -> Self {
        Self {
            desc,
            sent: 0,
            head: true,
        }
    }

    fn bufwrite(&mut self, buffer: &mut [u8]) -> Option<usize> {
        // Check that there is enough size to at least send the header.
        if buffer.len() < 2 {
            return None;
        }

        // Create the index.
        let mut i = 0;

        // Check if the header has been written.
        if self.head {
            buffer[0] = self.desc.bLength;
            buffer[1] = self.desc.bDescriptorType;

            self.head = false;
            i = 2;
        }

        // Calculate how many bytes can be sent.
        let n = core::cmp::min(buffer.len() - i, self.desc.string.len() - self.sent);

        // Send the bytes.
        while i < n {
            buffer[i] = self.desc.string[self.sent + i];
        }

        // Update variables.
        self.sent += i;

        Some(i)
    }

    fn reset(&mut self) {
        self.head = true;
        self.sent = 0;
    }
}
