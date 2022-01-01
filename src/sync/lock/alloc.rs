//! System Lock synchronization.
//! Reserves Spinlock 31 as system lock, which is used to moderate concurrent
//! access to shared resources of the RP2040.



use crate::prelude::*;

use core::ptr::{
    read_volatile as read,
    write_volatile as write,
};



pub struct AllocatorLock;

impl PowerState for AllocatorLock {
    #[inline(always)]
    fn poweron(&mut self) {}
    #[inline(always)]
    fn poweroff(&mut self) -> Result<(), SystemError> { Ok(()) }
    #[inline(always)]
    fn reset(&mut self) -> Result<(), SystemError> { Ok(()) }
}


impl Acquire for AllocatorLock {
    #[inline]
    fn acquire() -> Result<Self, SystemError> {
        // Volatile read on the lock.
        match unsafe { read(0xD0000178 as *const u32) } {
            0 => Err( SystemError::NoSystemLock ),
            _ => Ok( Self ),
        }
    }
}


impl Release for AllocatorLock {
    #[inline(always)]
    fn release(&mut self) {
        // Volatile write on the lock.
        unsafe { write(0xD0000178 as *mut u32, 1) }
    }
}

impl Drop for AllocatorLock {
    #[inline(always)]
    fn drop(&mut self) {
        // Volatile write on the lock.
        unsafe { write(0xD0000178 as *mut u32, 1) }
    }
}
