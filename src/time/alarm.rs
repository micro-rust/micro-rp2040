//! Module for the 4 64-bit alarms in the RP2040.


use crate::raw::AtomicRegister;

use micro::Register;

pub type Alarm0 = Alarm64Bits<0>;
pub type Alarm1 = Alarm64Bits<1>;
pub type Alarm2 = Alarm64Bits<2>;
pub type Alarm3 = Alarm64Bits<3>;


#[link_section = ".sysbss.ALARMSTATE"]
#[used]
static mut ALARMSTATE0: u32 = 0u32;

#[link_section = ".sysbss.ALARMSTATE"]
#[used]
static mut ALARMSTATE1: u32 = 0u32;


const ARMED  : u32 = 0x03;
const PAUSE  : u32 = 0x0C;
const DONE   : u32 = 0xF0;


/// Alarm A associated with the 64 bit Timer.
pub struct Alarm64Bits<const N: usize>;


/// Common trait for all alarms in the RP2040.
pub trait Alarm {
    /// Configures and enables the alarm.
    fn configure(&mut self, val: u32);

    /// Arms the alarm.
    fn arm(&mut self);

    /// Disarms the alarm.
    fn disarm(&mut self);

    /// Polls for the completion of the alarm.
    fn poll(&self) -> bool;
}


impl<const N: usize> Alarm for Alarm64Bits<N> {
    #[inline(always)]
    fn configure(&mut self, val: u32) {
        // Configure the alarm value and arm it.
        let alarm = unsafe { &mut *((0x40054010 + (4 * N)) as *mut AtomicRegister<u32>) };
        alarm.write(val);

        // Enable its interrupt.
        let inte = unsafe { &mut *(0x40054038 as *mut AtomicRegister<u32>) };
        inte.set(1 << (1 + N));

        // Configure its async handle.
        micro::asm::critical( || {
            match crate::sys::coreid() {
                0 => unsafe { ALARMSTATE0 |= 0x03 << (8 * N) },
                _ => unsafe { ALARMSTATE1 |= 0x03 << (8 * N) },
            }
        });
    }

    #[inline(always)]
    fn arm(&mut self) {
        let armed = unsafe { &mut *(0x40054020 as *mut AtomicRegister<u32>) };

        armed.set(1 << (1 + N));

        micro::asm::critical( || {
            match crate::sys::coreid() {
                0 => unsafe { ALARMSTATE0 |= 0x03 << (8 * N) },
                _ => unsafe { ALARMSTATE1 |= 0x03 << (8 * N) },
            }
        });
    }

    #[inline(always)]
    fn disarm(&mut self) {
        let armed = unsafe { &mut *(0x40054020 as *mut AtomicRegister<u32>) };

        armed.clear(1 << (1 + N));

        micro::asm::critical( || {
            match crate::sys::coreid() {
                0 => unsafe { ALARMSTATE0 &= !(0x03 << (8 * N)) },
                _ => unsafe { ALARMSTATE1 &= !(0x03 << (8 * N)) },
            }
        });
    }

    fn poll(&self) -> bool {
        let state = micro::asm::critical( || {
            match crate::sys::coreid() {
                0 => (unsafe { ALARMSTATE0 } >> (8 * N)) & 0xFF,
                _ => (unsafe { ALARMSTATE1 } >> (8 * N)) & 0xFF,
            }
        });

        match state {
            0xF3 => return true,
            _ => return false,
        }
    }
}


impl<const N: usize> Drop for Alarm64Bits<N> {
    fn drop(&mut self) {
        // Disarm the alarm.
        let armed = unsafe { &mut *(0x40054020 as *mut AtomicRegister<u32>) };
        armed.clear(1 << (1 + N));

        // Disable its interrupt.
        let inte = unsafe { &mut *(0x40054038 as *mut AtomicRegister<u32>) };
        inte.clear(1 << (1 + N));

        // Clear the async handle.
        micro::asm::critical( || {
            match crate::sys::coreid() {
                0 => unsafe { ALARMSTATE0 &= !(0xFF << (8 * N)) },
                _ => unsafe { ALARMSTATE1 &= !(0xFF << (8 * N)) },
            }
        });

        // Release the resource.
    }
}