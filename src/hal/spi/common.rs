//! Common utilities for SPI.



/// SPI Configuration struct.
#[repr(C)]
pub struct SPIConfig {
    pub(super) regs: u32,
    pub(super) baud: u32,
}

impl SPIConfig {
    /// Static initializer for the Motorola protocol.
    /// By default this is created in SPH 0 and SPO 0.
    #[inline(always)]
    pub const fn motorola(cpha: u8, cpol: u8) -> Self {
        let regs = ((cpha & 1) << 7) | ((cpol & 1) << 6);

        SPIConfig { regs: regs as u32, baud: 0, }
    }

    /// Static initializer for TI Synchronous protocol.
    #[inline(always)]
    pub const fn ti() -> Self {
        SPIConfig { regs: 0b01 << 4, baud: 0, }
    }

    /// Static initializer for National Microwire protocol.
    #[inline(always)]
    pub const fn microwire() -> Self {
        SPIConfig { regs: 0b10 << 4, baud: 0, }
    }

    /// Sets the frame size.
    #[inline(always)]
    pub const fn framesize(mut self, size: SPIFrameSize) -> Self {
        self.regs &= !0xF;
        self.regs |= size as u32;
        self
    }

    /// Sets master mode.
    #[inline(always)]
    pub const fn master(mut self) -> Self {
        self.regs &= !(0xF << 16);
        self
    }

    /// Sets slave mode and enables the output if requested.
    #[inline(always)]
    pub const fn slave(mut self, output: bool) -> Self {
        if output {
            self.regs &= !(0xF << 16);
            self.regs |= 0b0100 << 16;
        } else {
            self.regs |= 0b1100 << 16;
        }

        self
    }

    /// Sets desired baudrate.
    #[inline(always)]
    pub const fn baudrate(mut self, baud: u32) -> Self {
        self.baud = baud;
        self
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameFormat {
    /// Motorola SPI frame format.
    Motorola = 0b00,

    /// TI Synchronous Serial frame format.
    TISerial = 0b01,

    /// National Microwire frame format.
    Microwire = 0b10,
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SPIFrameSize {
    Bits4  = 0b0011,
    Bits5  = 0b0100,
    Bits6  = 0b0101,
    Bits7  = 0b0110,
    Bits8  = 0b0111,
    Bits9  = 0b1000,
    Bits10 = 0b1001,
    Bits11 = 0b1010,
    Bits12 = 0b1011,
    Bits13 = 0b1100,
    Bits14 = 0b1101,
    Bits15 = 0b1110,
    Bits16 = 0b1111,
}


#[inline(never)]
pub(super) fn bitrate(baudrate: u32) -> (u32, u32, u32) {
    use crate::math::UInt32;

    // Get peripheral frequency.
    let peripheral = crate::sys::Clocks::peripheral();

    // Find the smallest prescale value possible.
    let mut psc = u32::from( UInt32::from(peripheral) / (baudrate * 256) );

    psc = match psc {
        0 | 1 => 2,
        255.. => 254,
        n => n,
    };

    // Find largest postdivider.
    let mut pdv = u32::from( UInt32::from(peripheral) / (baudrate * psc) );

    pdv = match pdv {
        1..=256 => pdv - 1,
        0 => 0,
        257.. => 255,
    };

    // Get final baudrate.
    let baud = UInt32::from(peripheral) / (psc * (1 + pdv));

    (psc, pdv, baud.into())
}
