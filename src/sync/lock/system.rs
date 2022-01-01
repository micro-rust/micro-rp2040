//! System Lock synchronization.
//! Reserves Spinlock 31 as system lock, which is used to moderate concurrent
//! access to shared resources of the RP2040.



use crate::prelude::*;

use core::ptr::{
    read_volatile as read,
    write_volatile as write,
};


pub(crate) struct Syslock;

impl Acquire for Syslock {
    #[inline]
    fn acquire() -> Result<Self, SystemError> {
        // Volatile read on the lock.
        match unsafe { read(0xD000017C as *const u32) } {
            0 => Err( SystemError::NoSystemLock ),
            _ => Ok( Self ),
        }
    }
}


impl Release for Syslock {
    #[inline(always)]
    fn release(&mut self) -> Result<(), SystemError> {
        // Volatile write on the lock.
        unsafe { write(0xD000017C as *mut u32, 1); }
        Ok(())
    }
}

impl Drop for Syslock {
    #[inline(always)]
    fn drop(&mut self) {
        // Volatile write on the lock.
        unsafe { write(0xD000017C as *mut u32, 1) }
    }
}
