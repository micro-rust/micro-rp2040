//! String Descriptor Zero.
//! 
//! How to create a String Descriptor Zero.
//! 
//! ```
//! // First create the String Descriptors.
//! static MAINSTR: StringDescriptor = { ... };
//! 
//! // The create the USB Language descriptors.
//! // SPANISH will be wLang 1, so it's created before chaining to the wLang 0.
//! static SPANISH: USBLanguageDescriptor = USBLanguageDescriptor::new(0x040A);
//! 
//! // ENGLISH will be wLang 1. Remember that chaining a wLang to another will freeze the chained wLang.
//! static ENGLISH: USBLanguageDescriptor = USBLanguageDescriptor::new(0x0409)
//!     .chain(&SPANISH);
//! 
//! 
//! // Create the String Descriptor Zero.
//! static STRING0 : StringDescriptorZero = StringDescriptorZero::create(&ENGLISH, &MAINSTR);
//! ```



use super::super::{ Descriptor, DescriptorWriter, StringDescriptor };
use super::USBLanguageDescriptor;



pub struct StringDescriptorZero<'a> {
    /// Size of descriptor in bytes.
    pub(super) bLength: u8,

    /// Descriptor Type.
    bDescriptorType: u8,

    /// Supported Language Code Zero.
    lang: &'a USBLanguageDescriptor<'a>,

    /// First string.
    strings: &'a StringDescriptor<'a>,
}

impl<'a> StringDescriptorZero<'a> {
    /// Creates a new String Descriptor Zero.
    pub const fn create(lang: &'a USBLanguageDescriptor, strings: &'a StringDescriptor<'a>) -> Self {
        Self {
            bLength: lang.addsize(2) as u8,
            bDescriptorType: 0x03,
            lang,
            strings,
        }
    }
}

impl<'a> Descriptor for StringDescriptorZero<'a> {
    type Writer = StringDescriptorZeroWriter<'a>;
}





/// Byte Writer for the String Descriptor Zero.
pub struct StringDescriptorZeroWriter<'a> {
    /// Reference to the String Descriptor Zero.
    desc: &'a StringDescriptorZero<'a>,

    /// Optional pointer to the current wLang descriptor.
    lang: Option<&'a USBLanguageDescriptor<'a>>,

    /// Flag to check if the header has been written.
    head: bool,
}

impl<'a> DescriptorWriter for StringDescriptorZeroWriter<'a> {
    type Descriptor = StringDescriptorZero<'a>;

    fn create(desc: &'static Self::Descriptor) -> Self {
        Self {
            desc,
            lang: Some(desc.lang),
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

        // Check that there are more wLang to send and there is space in the buffer.
        while let Some(l) = self.lang {
            // Check that there is space in the buffer.
            if (buffer.len() - i - 1) < 2 {
                return Some(i);
            }

            // Write the lang into the buffer.
            buffer[i+0] = ((l.wLang >> 0) & 0xFF) as u8;
            buffer[i+1] = ((l.wLang >> 8) & 0xFF) as u8;

            self.lang = l.next;
            i += 2;
        }

        Some(i)
    }

    fn reset(&mut self) {
        self.head = true;
        self.lang = Some(self.desc.lang);
    }
}
