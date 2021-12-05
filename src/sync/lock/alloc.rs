//! System Lock synchronization.
//! Reserves Spinlock 31 as system lock, which is used to moderate concurrent
//! access to shared resources of the RP2040.



use crate::prelude::*;



pub struct AllocatorLock;

impl SystemResource for AllocatorLock {
    /// Acquires the lock if it's available.
    #[inline(always)]
    fn acquire() -> Result<Self, SystemError> {
        let lock = unsafe { &mut *(0xD0000178 as *mut SIORegister<u32>) };

        match lock.read() {
            0 => Err( SystemError::LockUnavailable ),
            _ => Ok( Self ),
        }
    }

    /// Releases the AllocatorLock.
    #[inline(always)]
    fn release(&mut self) {
        let lock = unsafe { &mut *(0xD0000178 as *mut SIORegister<u32>) };

        lock.write(1);
    }
}

impl Drop for AllocatorLock {
    fn drop(&mut self) {
        self.release()
    }
}
