//! Power subsystem.
//! Allows for power management, powering on and off different parts of the
//! RP2040 and control of voltage levels and brownout detection.


#![allow(non_camel_case_types)]


use crate::prelude::*;


pub struct PowerSystem;

impl PowerSystem {
    /// Overvolts the RP2040 to the given non-nominal voltage.
    /// UNSAFETY: Overvolting can cause damage to the MCU.
    pub unsafe fn overvolt(v: Overvoltage) -> Result<(), Error> {
        Self::voltage(v as u8)
    }


    /// Undervolts the RP2040 to the given non-nominal voltage.
    /// UNSAFETY: Undervolting can cause a brownout.
    pub unsafe fn undervolt(v: Undervoltage) -> Result<(), Error> {
        Self::voltage(v as u8)
    }

    /// Inner function to set the voltage.
    unsafe fn voltage(v: u8) -> Result<(), Error> {
        match Syslock::acquire() {
            Ok(_) => {
                let vreg = &mut *(0x40064000 as *mut AtomicRegister<u32>);

                let val = (u32::from(v) << 4) | 1;

                vreg.write(val);

                Ok(())
            },

            _ => Err( Error::System( SystemError::NoSystemLock ) ),
        }
    }

    /// Sets the nominal voltage.
    #[inline]
    pub fn nominal() {
        let vreg = unsafe { &mut *(0x40064000 as *mut AtomicRegister<u32>) };

        let val = (0b1011 << 4) | 1;

        vreg.write( val )
    }

    /// Sets the given borwnout level.
    pub unsafe fn brownout(level: Option<BrownoutLevel>) -> Result<(), Error> {
        let bod = &mut *(0x40064004 as *mut AtomicRegister<u32>);

        match Syslock::acquire() {
            Ok(_) => match level {
                None => { Ok( bod.clear(1) ) },
                Some(l) => Ok( bod.write( u32::from(l) | 1 ) ),
            },

            _ => Err( Error::System( SystemError::NoSystemLock ) ),
        }
    }

    /// Powers on the given domain.
    #[inline]
    pub fn poweron(id: PowerDomain) {
        let psm = unsafe { &mut *(0x40010000 as *mut [AtomicRegister<u32>; 4]) };

        psm[1].clear(u32::from(id));
        psm[0].set(u32::from(id))
    }

    /// Powers off the given domain.
    #[inline]
    pub unsafe fn poweroff(id: PowerDomain) -> Result<(), Error> {
        let psm = &mut *(0x40010000 as *mut [AtomicRegister<u32>; 4]);

        match Syslock::acquire() {
            Ok(_) => {
                psm[0].clear(u32::from(id));
                psm[1].set(u32::from(id));

                Ok(())
            },

            _ => Err( Error::System( SystemError::NoSystemLock ) ),
        }
    }

    /// Indicates if the given domain is powered on.
    /// This can return `false` for a while until the whole domain wakes up.
    #[inline]
    pub fn enabled(id: PowerDomain) -> bool {
        let psm = unsafe { &mut *(0x40010000 as *mut [AtomicRegister<u32>; 4]) };
        ( psm[3].read() & u32::from(id) ) == u32::from(id)
    }

    /// Indicates if the given domain is powered off.
    /// This can return `true` for a while until the whole domain wakes up.
    #[inline]
    pub fn disabled(id: PowerDomain) -> bool {
        let psm = unsafe { &mut *(0x40010000 as *mut [AtomicRegister<u32>; 4]) };
        ( psm[3].read() & u32::from(id) ) != u32::from(id)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Overvoltage {
    MilliVolts_1150 = 0b1100,
    MilliVolts_1200 = 0b1101,
    MilliVolts_1250 = 0b1110,
    MilliVolts_1300 = 0b1111,
}

impl core::convert::From<Overvoltage> for u32 {
    fn from(b: Overvoltage) -> u32 {
        b as u8 as u32
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Undervoltage {
    MilliVolts_0800 = 0b0101,
    MilliVolts_0850 = 0b0110,
    MilliVolts_0900 = 0b0111,
    MilliVolts_0950 = 0b1000,
    MilliVolts_1000 = 0b1001,
    MilliVolts_1050 = 0b1010,
}

impl core::convert::From<Undervoltage> for u32 {
    fn from(b: Undervoltage) -> u32 {
        b as u8 as u32
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BrownoutLevel {
    MilliVolts_0473 = 0b0000,
    MilliVolts_0516 = 0b0001,
    MilliVolts_0559 = 0b0010,
    MilliVolts_0602 = 0b0011,
    MilliVolts_0645 = 0b0100,
    MilliVolts_0688 = 0b0101,
    MilliVolts_0731 = 0b0110,
    MilliVolts_0774 = 0b0111,

    MilliVolts_0817 = 0b1000,
    MilliVolts_0860 = 0b1001,
    MilliVolts_0903 = 0b1010,
    MilliVolts_0946 = 0b1011,
    MilliVolts_0989 = 0b1100,
    MilliVolts_1032 = 0b1101,
    MilliVolts_1075 = 0b1110,
    MilliVolts_1118 = 0b1111,
}

impl core::convert::From<BrownoutLevel> for u32 {
    fn from(b: BrownoutLevel) -> u32 {
        b as u8 as u32
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PowerDomain(u32);

impl PowerDomain {
    pub const ROSC : PowerDomain = PowerDomain(1 << 0);

    pub const XOSC : PowerDomain = PowerDomain(1 << 1);

    pub const CLOCKS : PowerDomain = PowerDomain(1 << 2);

    pub const RESETS : PowerDomain = PowerDomain(1 << 3);

    pub const BUSFRABIC : PowerDomain = PowerDomain(1 << 4);

    pub const ROM : PowerDomain = PowerDomain(1 << 5);

    pub const SRAM0 : PowerDomain = PowerDomain(1 << 6);
    pub const SRAM1 : PowerDomain = PowerDomain(1 << 7);
    pub const SRAM2 : PowerDomain = PowerDomain(1 << 8);
    pub const SRAM3 : PowerDomain = PowerDomain(1 << 9);
    pub const SRAM4 : PowerDomain = PowerDomain(1 << 10);
    pub const SRAM5 : PowerDomain = PowerDomain(1 << 11);

    pub const XIP : PowerDomain = PowerDomain(1 << 12);

    pub const VREG : PowerDomain = PowerDomain(1 << 13);

    pub const SIO : PowerDomain = PowerDomain(1 << 14);

    pub const PROC0 : PowerDomain = PowerDomain(1 << 15);
    pub const PROC1 : PowerDomain = PowerDomain(1 << 16);

    /// XORs the ID.
    pub const fn inverse(&self) -> PowerDomain {
        PowerDomain(self.0 ^ 0xFFFFFFFF)
    }
}

impl core::ops::Add<Self> for PowerDomain {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        PowerDomain(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<Self> for PowerDomain {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        PowerDomain(self.0 | rhs.0)
    }
}

impl core::ops::AddAssign<Self> for PowerDomain {
    fn add_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl core::ops::BitOrAssign<Self> for PowerDomain {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl core::convert::From<PowerDomain> for u32 {
    fn from(x: PowerDomain) -> u32 {
        x.0
    }
}
