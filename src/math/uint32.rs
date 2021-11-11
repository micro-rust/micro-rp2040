//! u32 support for the Hardware divider.


use crate::raw::SIORegister;

use core::cmp::*;
use core::convert::*;
use core::ops::*;

use micro::Register;


#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct UInt32(u32);


impl UInt32 {
    /// No op transformation of u32 into UInt32.
    #[inline(always)]
    pub const fn new(i: u32) -> UInt32 {
        UInt32(i)
    }
}

impl<I> Add<I> for UInt32 where u32: From<I> {
    type Output = UInt32;

    fn add(self, rhs: I) -> UInt32 {
        UInt32( self.0 + u32::from(rhs) )
    }
}

impl<I> AddAssign<I> for UInt32 where u32: From<I> {
    fn add_assign(&mut self, rhs: I) {
        self.0 += u32::from(rhs)
    }
}

impl<I> Sub<I> for UInt32 where u32: From<I> {
    type Output = UInt32;

    fn sub(self, rhs: I) -> UInt32 {
        UInt32( self.0 - u32::from(rhs) )
    }
}

impl<I> SubAssign<I> for UInt32 where u32: From<I> {
    fn sub_assign(&mut self, rhs: I) {
        self.0 -= u32::from(rhs)
    }
}

impl<I> Mul<I> for UInt32 where u32: From<I> {
    type Output = UInt32;

    fn mul(self, rhs: I) -> UInt32 {
        UInt32( self.0 * u32::from(rhs) )
    }
}

impl<I> MulAssign<I> for UInt32 where u32: From<I> {
    fn mul_assign(&mut self, rhs: I) {
        self.0 *= u32::from(rhs)
    }
}

impl<I> BitAnd<I> for UInt32 where u32: From<I> {
    type Output = UInt32;

    fn bitand(self, rhs: I) -> UInt32 {
        UInt32( self.0 & u32::from(rhs) )
    }
}

impl<I> BitAndAssign<I> for UInt32 where u32: From<I> {
    fn bitand_assign(&mut self, rhs: I) {
        self.0 &= u32::from(rhs)
    }
}

impl<I> BitOr<I> for UInt32 where u32: From<I> {
    type Output = UInt32;

    fn bitor(self, rhs: I) -> UInt32 {
        UInt32( self.0 | u32::from(rhs) )
    }
}

impl<I> BitOrAssign<I> for UInt32 where u32: From<I> {
    fn bitor_assign(&mut self, rhs: I) {
        self.0 |= u32::from(rhs)
    }
}

impl<I> BitXor<I> for UInt32 where u32: From<I> {
    type Output = UInt32;

    fn bitxor(self, rhs: I) -> UInt32 {
        UInt32( self.0 ^ u32::from(rhs) )
    }
}

impl<I> BitXorAssign<I> for UInt32 where u32: From<I> {
    fn bitxor_assign(&mut self, rhs: I) {
        self.0 ^= u32::from(rhs)
    }
}

impl Not for UInt32 {
    type Output = UInt32;

    fn not(self) -> UInt32 {
        UInt32( !self.0 )
    }
}

impl<I> Shl<I> for UInt32 where u32: From<I> {
    type Output = UInt32;

    fn shl(self, rhs: I) -> UInt32 {
        UInt32( self.0 << u32::from(rhs) )
    }
}

impl<I> ShlAssign<I> for UInt32 where u32: From<I> {
    fn shl_assign(&mut self, rhs: I) {
        self.0 <<= u32::from(rhs)
    }
}

impl<I> Shr<I> for UInt32 where u32: From<I> {
    type Output = UInt32;

    fn shr(self, rhs: I) -> UInt32 {
        UInt32( self.0 >> u32::from(rhs) )
    }
}

impl<I> ShrAssign<I> for UInt32 where u32: From<I> {
    fn shr_assign(&mut self, rhs: I) {
        self.0 >>= u32::from(rhs)
    }
}


impl<I> Div<I> for UInt32 where u32: From<I> {
    type Output = UInt32;

    #[inline(never)]
    fn div(self, rhs: I) -> UInt32 {
        let sdividend = unsafe { &mut *(0xD0000068 as *mut SIORegister<u32>) };
        let sdivisor  = unsafe { &mut *(0xD000006C as *mut SIORegister<u32>) };

        let quotient  = unsafe { & *(0xD0000070 as *const SIORegister<u32>) };
        let remainder = unsafe { & *(0xD0000074 as *const SIORegister<u32>) };

        sdividend.write(self.0);
        sdivisor.write(u32::from(rhs));

        delay();

        remainder.read();
        UInt32::new( quotient.read() )
    }
}

impl<I> DivAssign<I> for UInt32 where u32: From<I> {
    #[inline(never)]
    fn div_assign(&mut self, rhs: I) {
        let sdividend = unsafe { &mut *(0xD0000068 as *mut SIORegister<u32>) };
        let sdivisor  = unsafe { &mut *(0xD000006C as *mut SIORegister<u32>) };

        let quotient  = unsafe { & *(0xD0000070 as *const SIORegister<u32>) };
        let remainder = unsafe { & *(0xD0000074 as *const SIORegister<u32>) };

        sdividend.write(self.0);
        sdivisor.write(u32::from(rhs));

        delay();

        remainder.read();
        self.0 = quotient.read();
    }
}

impl<I> Rem<I> for UInt32 where u32: From<I> {
    type Output = UInt32;

    #[inline(never)]
    fn rem(self, rhs: I) -> UInt32 {
        let sdividend = unsafe { &mut *(0xD0000068 as *mut SIORegister<u32>) };
        let sdivisor  = unsafe { &mut *(0xD000006C as *mut SIORegister<u32>) };

        let quotient  = unsafe { & *(0xD0000070 as *const SIORegister<u32>) };
        let remainder = unsafe { & *(0xD0000074 as *const SIORegister<u32>) };

        sdividend.write(self.0);
        sdivisor.write(u32::from(rhs));

        delay();

        let rem = remainder.read();
        quotient.read();

        UInt32( rem )
    }
}

impl<I> RemAssign<I> for UInt32 where u32: From<I> {
    #[inline(never)]
    fn rem_assign(&mut self, rhs: I) {
        let sdividend = unsafe { &mut *(0xD0000068 as *mut SIORegister<u32>) };
        let sdivisor  = unsafe { &mut *(0xD000006C as *mut SIORegister<u32>) };

        let quotient  = unsafe { & *(0xD0000070 as *const SIORegister<u32>) };
        let remainder = unsafe { & *(0xD0000074 as *const SIORegister<u32>) };

        sdividend.write(self.0);
        sdivisor.write(u32::from(rhs));

        delay();

        self.0 = remainder.read();
        quotient.read();
    }
}

impl PartialEq<u32> for UInt32 {
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }

    fn ne(&self, other: &u32) -> bool {
        self.0.ne(other)
    }
}

impl PartialEq<UInt32> for UInt32 {
    fn eq(&self, other: &UInt32) -> bool {
        self.0.eq(&other.0)
    }

    fn ne(&self, other: &UInt32) -> bool {
        self.0.ne(&other.0)
    }
}

impl Eq for UInt32 {}

impl PartialOrd<u32> for UInt32 {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }

    fn lt(&self, other: &u32) -> bool {
        self.0.lt(other)
    }

    fn le(&self, other: &u32) -> bool {
        self.0.le(other)
    }

    fn gt(&self, other: &u32) -> bool {
        self.0.gt(other)
    }

    fn ge(&self, other: &u32) -> bool {
        self.0.ge(other)
    }
}

impl PartialOrd<UInt32> for UInt32 {
    fn partial_cmp(&self, other: &UInt32) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }

    fn lt(&self, other: &UInt32) -> bool {
        self.0.lt(&other.0)
    }

    fn le(&self, other: &UInt32) -> bool {
        self.0.le(&other.0)
    }

    fn gt(&self, other: &UInt32) -> bool {
        self.0.gt(&other.0)
    }

    fn ge(&self, other: &UInt32) -> bool {
        self.0.ge(&other.0)
    }
}

impl Ord for UInt32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}


impl From<u32> for UInt32 {
    #[inline(always)]
    fn from(i: u32) -> UInt32 {
        UInt32(i)
    }
}

impl From<UInt32> for u32 {
    #[inline(always)]
    fn from(i: UInt32) -> u32 {
        i.0
    }
}



#[inline(always)]
fn delay() {
unsafe { asm!("
    b 1f
1:  b 1f
1:  b 1f
1:  b 1f
1:
") }
}