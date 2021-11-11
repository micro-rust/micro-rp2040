//! i32 support for the Hardware divider.


use crate::raw::SIORegister;

use core::cmp::*;
use core::convert::*;
use core::ops::*;

use micro::Register;


#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Int32(i32);


impl Int32 {
    /// No op transformation of i32 into Int32.
    #[inline(always)]
    pub const fn new(i: i32) -> Int32 {
        Int32(i)
    }
}

impl<I> Add<I> for Int32 where i32: From<I> {
    type Output = Int32;

    fn add(self, rhs: I) -> Int32 {
        Int32( self.0 + i32::from(rhs) )
    }
}

impl<I> AddAssign<I> for Int32 where i32: From<I> {
    fn add_assign(&mut self, rhs: I) {
        self.0 += i32::from(rhs)
    }
}

impl<I> Sub<I> for Int32 where i32: From<I> {
    type Output = Int32;

    fn sub(self, rhs: I) -> Int32 {
        Int32( self.0 - i32::from(rhs) )
    }
}

impl<I> SubAssign<I> for Int32 where i32: From<I> {
    fn sub_assign(&mut self, rhs: I) {
        self.0 -= i32::from(rhs)
    }
}

impl<I> Mul<I> for Int32 where i32: From<I> {
    type Output = Int32;

    fn mul(self, rhs: I) -> Int32 {
        Int32( self.0 * i32::from(rhs) )
    }
}

impl<I> MulAssign<I> for Int32 where i32: From<I> {
    fn mul_assign(&mut self, rhs: I) {
        self.0 *= i32::from(rhs)
    }
}

impl<I> BitAnd<I> for Int32 where i32: From<I> {
    type Output = Int32;

    fn bitand(self, rhs: I) -> Int32 {
        Int32( self.0 & i32::from(rhs) )
    }
}

impl<I> BitAndAssign<I> for Int32 where i32: From<I> {
    fn bitand_assign(&mut self, rhs: I) {
        self.0 &= i32::from(rhs)
    }
}

impl<I> BitOr<I> for Int32 where i32: From<I> {
    type Output = Int32;

    fn bitor(self, rhs: I) -> Int32 {
        Int32( self.0 | i32::from(rhs) )
    }
}

impl<I> BitOrAssign<I> for Int32 where i32: From<I> {
    fn bitor_assign(&mut self, rhs: I) {
        self.0 |= i32::from(rhs)
    }
}

impl<I> BitXor<I> for Int32 where i32: From<I> {
    type Output = Int32;

    fn bitxor(self, rhs: I) -> Int32 {
        Int32( self.0 ^ i32::from(rhs) )
    }
}

impl<I> BitXorAssign<I> for Int32 where i32: From<I> {
    fn bitxor_assign(&mut self, rhs: I) {
        self.0 ^= i32::from(rhs)
    }
}

impl Not for Int32 {
    type Output = Int32;

    fn not(self) -> Int32 {
        Int32( !self.0 )
    }
}

impl<I> Shl<I> for Int32 where i32: From<I> {
    type Output = Int32;

    fn shl(self, rhs: I) -> Int32 {
        Int32( self.0 << i32::from(rhs) )
    }
}

impl<I> ShlAssign<I> for Int32 where i32: From<I> {
    fn shl_assign(&mut self, rhs: I) {
        self.0 <<= i32::from(rhs)
    }
}

impl<I> Shr<I> for Int32 where i32: From<I> {
    type Output = Int32;

    fn shr(self, rhs: I) -> Int32 {
        Int32( self.0 >> i32::from(rhs) )
    }
}

impl<I> ShrAssign<I> for Int32 where i32: From<I> {
    fn shr_assign(&mut self, rhs: I) {
        self.0 >>= i32::from(rhs)
    }
}


impl<I> Div<I> for Int32 where i32: From<I> {
    type Output = Int32;

    fn div(self, rhs: I) -> Int32 {
        let sdividend = unsafe { &mut *(0xD0000068 as *mut SIORegister<i32>) };
        let sdivisor  = unsafe { &mut *(0xD000006C as *mut SIORegister<i32>) };

        let quotient  = unsafe { & *(0xD0000070 as *const SIORegister<i32>) };
        let remainder = unsafe { & *(0xD0000074 as *const SIORegister<i32>) };

        sdividend.write(self.0);
        sdivisor.write(i32::from(rhs));

        delay();

        remainder.read();
        Int32::new( quotient.read() )
    }
}

impl<I> DivAssign<I> for Int32 where i32: From<I> {
    fn div_assign(&mut self, rhs: I) {
        let sdividend = unsafe { &mut *(0xD0000068 as *mut SIORegister<i32>) };
        let sdivisor  = unsafe { &mut *(0xD000006C as *mut SIORegister<i32>) };

        let quotient  = unsafe { & *(0xD0000070 as *const SIORegister<i32>) };
        let remainder = unsafe { & *(0xD0000074 as *const SIORegister<i32>) };

        sdividend.write(self.0);
        sdivisor.write(i32::from(rhs));

        delay();

        remainder.read();
        self.0 = quotient.read();
    }
}

impl<I> Rem<I> for Int32 where i32: From<I> {
    type Output = Int32;

    fn rem(self, rhs: I) -> Int32 {
        let sdividend = unsafe { &mut *(0xD0000068 as *mut SIORegister<i32>) };
        let sdivisor  = unsafe { &mut *(0xD000006C as *mut SIORegister<i32>) };

        let quotient  = unsafe { & *(0xD0000070 as *const SIORegister<i32>) };
        let remainder = unsafe { & *(0xD0000074 as *const SIORegister<i32>) };

        sdividend.write(self.0);
        sdivisor.write(i32::from(rhs));

        delay();

        let rem = remainder.read();
        quotient.read();

        Int32( rem )
    }
}

impl<I> RemAssign<I> for Int32 where i32: From<I> {
    fn rem_assign(&mut self, rhs: I) {
        let sdividend = unsafe { &mut *(0xD0000068 as *mut SIORegister<i32>) };
        let sdivisor  = unsafe { &mut *(0xD000006C as *mut SIORegister<i32>) };

        let quotient  = unsafe { & *(0xD0000070 as *const SIORegister<i32>) };
        let remainder = unsafe { & *(0xD0000074 as *const SIORegister<i32>) };

        sdividend.write(self.0);
        sdivisor.write(i32::from(rhs));

        delay();

        self.0 = remainder.read();
        quotient.read();
    }
}


impl PartialEq<i32> for Int32 {
    fn eq(&self, other: &i32) -> bool {
        self.0.eq(other)
    }

    fn ne(&self, other: &i32) -> bool {
        self.0.ne(other)
    }
}

impl PartialEq<Int32> for Int32 {
    fn eq(&self, other: &Int32) -> bool {
        self.0.eq(&other.0)
    }

    fn ne(&self, other: &Int32) -> bool {
        self.0.ne(&other.0)
    }
}

impl Eq for Int32 {}

impl PartialOrd<i32> for Int32 {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }

    fn lt(&self, other: &i32) -> bool {
        self.0.lt(other)
    }

    fn le(&self, other: &i32) -> bool {
        self.0.le(other)
    }

    fn gt(&self, other: &i32) -> bool {
        self.0.gt(other)
    }

    fn ge(&self, other: &i32) -> bool {
        self.0.ge(other)
    }
}

impl PartialOrd<Int32> for Int32 {
    fn partial_cmp(&self, other: &Int32) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }

    fn lt(&self, other: &Int32) -> bool {
        self.0.lt(&other.0)
    }

    fn le(&self, other: &Int32) -> bool {
        self.0.le(&other.0)
    }

    fn gt(&self, other: &Int32) -> bool {
        self.0.gt(&other.0)
    }

    fn ge(&self, other: &Int32) -> bool {
        self.0.ge(&other.0)
    }
}

impl Ord for Int32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}


impl From<i32> for Int32 {
    #[inline(always)]
    fn from(i: i32) -> Int32 {
        Int32(i)
    }
}

impl From<Int32> for i32 {
    #[inline(always)]
    fn from(i: Int32) -> i32 {
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