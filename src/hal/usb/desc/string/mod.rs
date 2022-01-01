//! String Descriptors module.

mod string;
mod zero;

pub use self::string::{ StringDescriptor, StringDescriptorWriter };
pub use self::zero::{ StringDescriptorZero, StringDescriptorZeroWriter };



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct USBLanguageDescriptor<'a> {
    /// Language Code.
    pub(self) wLang: u16,

    /// Optional linked langauge.
    pub(self) next: Option<&'a USBLanguageDescriptor<'a>>,
}

impl<'a> USBLanguageDescriptor<'a> {

    /// Language code for English (United States).
    pub const ENGLISH_US: u16 = 0x0409;

    /// Static initializer.
    pub const fn new(wLang: u16) -> USBLanguageDescriptor<'a> {
        USBLanguageDescriptor {
            wLang,
            next: None,
        }
    }

    /// Chains a new USB Language Descriptor to this one.
    pub const fn chain(mut self, next: &'a USBLanguageDescriptor) -> Self {
        self.next = Some(next);
        self
    }

    /// Adds its own size to the current size.
    pub const fn addsize(&self, mut size: usize) -> usize {
        // Add own size.
        size += 2;

        // Check for a linked language.
        match self.next {
            Some(next) => next.addsize(size),
            None => size,
        }
    }
}
