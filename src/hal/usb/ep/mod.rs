//! USB Endpoint module.

//mod control;


//pub use self::control::EPControl;

use core::ptr::{
    read_volatile as read,
    write_volatile as write,
};


#[link_section = ".sysbss0.USBEPHANDLES"]
#[used]
static mut USBEPHANDLES: [u8; 32] = [0u8; 32];




pub struct Endpoint<const EP: usize>;

impl<const EP: usize> Endpoint<EP> {
    /// Splits the Endpoint into Input and Output.
    #[inline(always)]
    pub fn split(&mut self) -> (EndpointIn<EP>, EndpointOut<EP>) {
        core::mem::forget(self);
        (EndpointIn::new(), EndpointOut::new())
    }
}


pub struct EndpointIn<const EP: usize>;

impl<const EP: usize> EndpointIn<EP> {
    /// Creates a new Enpoint Input.
    pub(self) fn new() -> Self {
        Self
    }

    /// Returns the pointer to the buffer control.
    #[inline(always)]
    pub fn ctrl(&mut self) -> *mut u32 {
        unsafe { (0x50100000 + (0x8 * EP)) as *mut u32 }
    }

    /// Returns the address to the DPRAM buffer.
    #[inline]
    pub fn buffer(&mut self) -> *mut u8 {
        unsafe { ((read(self.ptr()) & 0xFF) + 0x50100000) as *const u8 }
    }
}


pub struct EndpointOut<const EP: usize>;

impl<const EP: usize> EndpointOut<EP> {
    /// Creates a new Enpoint Output.
    pub(self) fn new() -> Self {
        Self
    }

    /// Returns the pointer to the buffer control.
    #[inline(always)]
    pub fn ctrl(&mut self) -> *mut u32 {
        unsafe { (0x50100004 + (0x8 * EP)) as *mut u32 }
    }

    /// Returns the address to the DPRAM buffer.
    #[inline]
    pub fn buffer(&mut self) -> *mut u8 {
        unsafe { ((read(self.ptr()) & 0xFF) + 0x50100000) as *const u8 }
    }
}



pub struct BulkEndpoint<const EP: usize>;

impl<const EP: usize> BulkEndpoint<EP> {
    /// Sends the given buffer throught the Bulk Endpoint.
    pub fn send(&mut self, data: &[u8]) -> Result<(), ()> {
        // Perform the checks.


    }
}


// This should trigger after each transfer or after an error.
pub(crate) fn usbhandler() {
    // Check which endpoint triggered the IRQ.

    // Get the handler associated with it.





    // Build a new Buffer Control.
    let mut ctrl = 0u32;

    // If the buffer is the last, set the flag.
    if rem < 64 { ctrl |= 1 << 14 }

    // 
}



#[repr(transparent)]
pub struct EndpointConfig(u32);


impl EndpointConfig {
    /// Enables / Disables the interrupt for every transferred buffer.
    #[inline(always)]
    pub const fn singleint(mut self, s: bool) -> mut Self {
        if s { self.0 |= 1 << 29 }
        else { self.0 &= !(1 << 29) }

        self
    }

    /// Enables / Disables the interrupt for every 2 transferred buffers.
    #[inline(always)]
    pub const fn doubleint(mut self, s: bool) -> mut Self {
        if s { self.0 |= 1 << 29 }
        else { self.0 &= !(1 << 29) }

        self
    }

    /// Sets the endpoint type to Control.
    #[inline(always)]
    pub const fn control(mut self) -> mut Self {
        self.0 &= !(0x3 << 26);
        self
    }

    /// Sets the endpoint type to Isochronous.
    #[inline(always)]
    pub const fn isochronous(mut self) -> mut Self {
        self.0 &= !(0x3 << 26);
        self.0 |= 0x1 << 26;
        self
    }

    /// Sets the endpoint type to Bulk.
    #[inline(always)]
    pub const fn bulk(mut self) -> mut Self {
        self.0 &= !(0x3 << 26);
        self.0 |= 0x2 << 26;
        self
    }

    /// Sets the endpoint type to Interrupt.
    #[inline(always)]
    pub const fn interrupt(mut self) -> mut Self {
        self.0 |= 0x3 << 26;
        self
    }

    /// Enables / Disables the interrupt on Stall.
    #[inline(always)]
    pub const fn stallint(mut self, s: bool) -> mut Self {
        if s { self.0 |= 1 << 17 }
        else { self.0 &= !(1 << 17) }

        self
    }

    /// Enables / Disables the interrupt on NAK.
    #[inline(always)]
    pub const fn nakint(mut self, s: bool) -> mut Self {
        if s { self.0 |= 1 << 16 }
        else { self.0 &= !(1 << 16) }

        self
    }

    /// Sets the address offset of the buffer for this endpoint.
    #[inline(always)]
    pub fn address(mut self, addr: u16) -> mut Self {
        self.0 &= !0xFFFF;
        self.0 |= (addr as 0xFFE0) as u32;

        self
    }
}
