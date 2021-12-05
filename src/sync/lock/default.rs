//! System Lock synchronization.
//! Gives access to all spinlocks for synchronization purposes.



use crate::prelude::*;



#[cfg(feature = "alloc")]
const MAX: usize = 29;

#[cfg(not(feature = "alloc"))]
const MAX: usize = 30;



/// Hardware spinlock. Can only access spinlocks 0-30.
/// Spinlock 31 is reserved for System use.
pub struct Spinlock<const N: usize>;


impl<const N: usize> SystemResource for Spinlock<N> {
    /// Acquires the lock if it's available.
    #[inline(always)]
    fn acquire() -> Result<Self, SystemError> {
        if N > 31 { panic!("Spinlocks higher than 31 do not exist.") }
        if N == 31 { panic!("Spinlock 31 is system reserved.") }

        #[cfg(feature = "alloc")]
        if N == 30 { panic!("Spinlock 31 is system reserved.") }

        let lock = unsafe { &mut *(0xD0000100 as *mut [SIORegister<u32>; MAX]) };

        match lock[N].read() {
            0 => Err( SystemError::LockUnavailable ),
            _ => Ok( Self ),
        }
    }

    /// Releases the Spinlock.
    #[inline(always)]
    fn release(&mut self) {
        let lock = unsafe { &mut *(0xD0000100 as *mut [SIORegister<u32>; MAX]) };

        lock[N].write(1);
    }
}

impl<const N: usize> Drop for Spinlock<N> {
    fn drop(&mut self) {
        self.release()
    }
}



pub type Spinlock0  = Spinlock< 0>;
pub type Spinlock1  = Spinlock< 1>;
pub type Spinlock2  = Spinlock< 2>;
pub type Spinlock3  = Spinlock< 3>;
pub type Spinlock4  = Spinlock< 4>;
pub type Spinlock5  = Spinlock< 5>;
pub type Spinlock6  = Spinlock< 6>;
pub type Spinlock7  = Spinlock< 7>;
pub type Spinlock8  = Spinlock< 8>;
pub type Spinlock9  = Spinlock< 9>;
pub type Spinlock10 = Spinlock<10>;
pub type Spinlock11 = Spinlock<11>;
pub type Spinlock12 = Spinlock<12>;
pub type Spinlock13 = Spinlock<13>;
pub type Spinlock14 = Spinlock<14>;
pub type Spinlock15 = Spinlock<15>;
pub type Spinlock16 = Spinlock<16>;
pub type Spinlock17 = Spinlock<17>;
pub type Spinlock18 = Spinlock<18>;
pub type Spinlock19 = Spinlock<19>;
pub type Spinlock20 = Spinlock<20>;
pub type Spinlock21 = Spinlock<21>;
pub type Spinlock22 = Spinlock<22>;
pub type Spinlock23 = Spinlock<23>;
pub type Spinlock24 = Spinlock<24>;
pub type Spinlock25 = Spinlock<25>;
pub type Spinlock26 = Spinlock<26>;
pub type Spinlock27 = Spinlock<27>;
pub type Spinlock28 = Spinlock<28>;
pub type Spinlock29 = Spinlock<29>;

#[cfg(not(feature = "alloc"))]
pub type Spinlock30 = Spinlock<30>;
