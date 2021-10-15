//! Resources module.


use crate::sync::Syslock;



#[link_section = ".systembss.RESOURCES"]
pub(crate) static mut RESOURCES : [u32; 8] = [0u32; 8];


pub trait SystemResource: Sized {
    const LOCKNUM: usize;
    const LOCKOFF: u8;

    /// Function to acquire the resource.
    fn acquire() -> Result<Self, ()>;
}

















#[link_section = ".systembss.RESOURCES.COMMS"]
static mut COMMS : u32 = 0x00000000;


#[link_section = ".systembss.RESOURCES.SYSTEM"]
static mut SYSTEM : u32 = 0x00000000;

#[link_section = ".systembss.RESOURCES.DMA"]
static mut DMA : u32 = 0x00000000;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    // COMM peripherals.

    UART0 = 0x0010,
    UART1 = 0x0020,

    SPI0  = 0x0040,
    SPI1  = 0x0080,

    I2C0  = 0x0100,
    I2C1  = 0x0200,

    PIO0  = 0x0400,
    PIO1  = 0x0800,

    USB   = 0x1000,
    SSI   = 0x2000,


    // System peripherals.
    
}

impl Resource {
    /// Returns `true` if the resource is a communication resource.
    #[inline(always)]
    pub fn comm(&self) -> bool {
        match *self as u32 & 0xF {
            0 => true,
            _ => false,
        }
    }

    /// Returns `true` if the resource is a system resource.
    #[inline(always)]
    pub fn system(&self) -> bool {
        match *self as u32 & 0xF {
            1 => true,
            _ => false,
        }
    }

    /// Returns `true` if the resource is a DMA resource.
    #[inline(always)]
    pub fn dma(&self) -> bool {
        match *self as u32 & 0xF {
            2 => true,
            _ => false,
        }
    }

    /// Reserves the given peripheral.
    pub fn reserve(&self) -> Result<(), ()> {
        if self.comm() {
            return self.__reserve__(unsafe { &mut COMMS });
        }

        if self.system() {
            return self.__reserve__(unsafe { &mut SYSTEM });
        }

        if self.dma() {
            return self.__reserve__(unsafe { &mut DMA });
        }

        Err(())
    }

    /// Reserves a communication resource.
    fn __reserve__(&self, lock: &'static mut u32) -> Result<(), ()> {
        match Syslock::acquire() {
            Some(_) => match *lock & (*self as u32) {
                0 => {
                    *lock |= (*self as u32) & !0xF;

                    Ok(())
                },
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    /// Releases the given peripheral.
    pub fn release(&self) {
        if self.comm() {
            self.__release__( unsafe { &mut COMMS } )
        }

        if self.system() {
            self.__release__( unsafe { &mut SYSTEM } )
        }

        if self.dma() {
            self.__release__( unsafe { &mut DMA } )
        }
    }

    /// Releases a communication resource.
    fn __release__(&self, lock: &'static mut u32) {
        loop {
            match Syslock::acquire() {
                Some(_) => { *lock &= !((*self as u32) & !0xF); return; },
                _ => (),
            }
        }
    }
}


