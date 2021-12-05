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

impl Add<Int32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn add(self, rhs: Int32) -> Int32 {
        Int32( self.0 + rhs.0 )
    }
}

impl AddAssign<Int32> for Int32{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Int32) {
        self.0 += rhs.0
    }
}

impl Add<i32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn add(self, rhs: i32) -> Int32 {
        Int32( self.0 + rhs )
    }
}

impl AddAssign<i32> for Int32{
    #[inline(always)]
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs
    }
}

impl Add<Int32> for i32 {
    type Output = Int32;

    #[inline(always)]
    fn add(self, rhs: Int32) -> Int32 {
        Int32( self + rhs.0 )
    }
}

impl AddAssign<Int32> for i32{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Int32) {
        *self += rhs.0
    }
}

impl Sub<Int32> for Int32{
    type Output = Int32;

    #[inline(always)]
    fn sub(self, rhs: Int32) -> Int32 {
        Int32( self.0 - rhs.0 )
    }
}

impl SubAssign<Int32> for Int32{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Int32) {
        self.0 -= rhs.0
    }
}

impl Sub<i32> for Int32{
    type Output = Int32;

    #[inline(always)]
    fn sub(self, rhs: i32) -> Int32 {
        Int32( self.0 - rhs )
    }
}

impl SubAssign<i32> for Int32{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: i32) {
        self.0 -= rhs
    }
}

impl Sub<Int32> for i32{
    type Output = Int32;

    #[inline(always)]
    fn sub(self, rhs: Int32) -> Int32 {
        Int32( self - rhs.0 )
    }
}

impl SubAssign<Int32> for i32{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Int32) {
        *self -= rhs.0
    }
}

impl Mul<Int32> for Int32{
    type Output = Int32;

    #[inline(always)]
    fn mul(self, rhs: Int32) -> Int32 {
        Int32( self.0 * rhs.0 )
    }
}

impl MulAssign<Int32> for Int32 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Int32) {
        self.0 *= rhs.0
    }
}

impl Mul<i32> for Int32{
    type Output = Int32;

    #[inline(always)]
    fn mul(self, rhs: i32) -> Int32 {
        Int32( self.0 * rhs )
    }
}

impl MulAssign<i32> for Int32 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: i32) {
        self.0 *= rhs
    }
}

impl Mul<Int32> for i32{
    type Output = Int32;

    #[inline(always)]
    fn mul(self, rhs: Int32) -> Int32 {
        Int32( self * rhs.0 )
    }
}

impl MulAssign<Int32> for i32 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Int32) {
        *self *= rhs.0
    }
}

impl BitAnd<Int32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn bitand(self, rhs: Int32) -> Int32 {
        Int32( self.0 & rhs.0 )
    }
}

impl BitAndAssign<Int32> for Int32 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Int32) {
        self.0 &= rhs.0
    }
}

impl BitAnd<i32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn bitand(self, rhs: i32) -> Int32 {
        Int32( self.0 & rhs )
    }
}

impl BitAndAssign<i32> for Int32 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: i32) {
        self.0 &= rhs
    }
}

impl BitAnd<Int32> for i32 {
    type Output = Int32;

    #[inline(always)]
    fn bitand(self, rhs: Int32) -> Int32 {
        Int32( self & rhs.0 )
    }
}

impl BitAndAssign<Int32> for i32 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Int32) {
        *self &= rhs.0
    }
}

impl BitOr<Int32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn bitor(self, rhs: Int32) -> Int32 {
        Int32( self.0 | rhs.0 )
    }
}

impl BitOrAssign<Int32> for Int32 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Int32) {
        self.0 |= rhs.0
    }
}

impl BitOr<i32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn bitor(self, rhs: i32) -> Int32 {
        Int32( self.0 | rhs )
    }
}

impl BitOrAssign<i32> for Int32 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: i32) {
        self.0 |= rhs
    }
}

impl BitOr<Int32> for i32 {
    type Output = Int32;

    #[inline(always)]
    fn bitor(self, rhs: Int32) -> Int32 {
        Int32( self | rhs.0 )
    }
}

impl BitOrAssign<Int32> for i32 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Int32) {
        *self |= rhs.0
    }
}

impl BitXor<Int32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn bitxor(self, rhs: Int32) -> Int32 {
        Int32( self.0 ^ rhs.0 )
    }
}

impl BitXorAssign<Int32> for Int32 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Int32) {
        self.0 ^= rhs.0
    }
}

impl BitXor<i32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn bitxor(self, rhs: i32) -> Int32 {
        Int32( self.0 ^ rhs )
    }
}

impl BitXorAssign<i32> for Int32 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: i32) {
        self.0 ^= rhs
    }
}

impl BitXor<Int32> for i32 {
    type Output = Int32;

    #[inline(always)]
    fn bitxor(self, rhs: Int32) -> Int32 {
        Int32( self ^ rhs.0 )
    }
}

impl BitXorAssign<Int32> for i32 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Int32) {
        *self ^= rhs.0
    }
}

impl Not for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn not(self) -> Int32 {
        Int32( !self.0 )
    }
}

impl Shl<Int32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn shl(self, rhs: Int32) -> Int32 {
        Int32( self.0 << rhs.0 )
    }
}

impl ShlAssign<Int32> for Int32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Int32) {
        self.0 <<= rhs.0
    }
}

impl Shl<u64> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn shl(self, rhs: u64) -> Int32 {
        Int32( self.0 << rhs )
    }
}

impl ShlAssign<u64> for Int32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u64) {
        self.0 <<= rhs
    }
}

impl Shl<Int32> for u64 {
    type Output = u64;

    #[inline(always)]
    fn shl(self, rhs: Int32) -> u64 {
        self << rhs.0
    }
}

impl ShlAssign<Int32> for u64 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Int32) {
        *self <<= rhs.0
    }
}

impl Shl<i32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn shl(self, rhs: i32) -> Int32 {
        Int32( self.0 << rhs )
    }
}

impl ShlAssign<i32> for Int32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: i32) {
        self.0 <<= rhs
    }
}

impl Shl<Int32> for i32 {
    type Output = i32;

    #[inline(always)]
    fn shl(self, rhs: Int32) -> i32 {
        self << rhs.0
    }
}

impl ShlAssign<Int32> for i32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Int32) {
        *self <<= rhs.0
    }
}

impl Shl<usize> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn shl(self, rhs: usize) -> Int32 {
        Int32( self.0 << rhs )
    }
}

impl ShlAssign<usize> for Int32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs
    }
}

impl Shl<Int32> for usize {
    type Output = usize;

    #[inline(always)]
    fn shl(self, rhs: Int32) -> usize {
        self << rhs.0
    }
}

impl ShlAssign<Int32> for usize {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Int32) {
        *self <<= rhs.0
    }
}

impl Shl<u16> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn shl(self, rhs: u16) -> Int32 {
        Int32( self.0 << rhs )
    }
}

impl ShlAssign<u16> for Int32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u16) {
        self.0 <<= rhs
    }
}

impl Shl<Int32> for u16 {
    type Output = u16;

    #[inline(always)]
    fn shl(self, rhs: Int32) -> u16 {
        self << rhs.0
    }
}

impl ShlAssign<Int32> for u16 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Int32) {
        *self <<= rhs.0
    }
}

impl Shl<u8> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn shl(self, rhs: u8) -> Int32 {
        Int32( self.0 << rhs )
    }
}

impl ShlAssign<u8> for Int32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs
    }
}

impl Shl<Int32> for u8 {
    type Output = u8;

    #[inline(always)]
    fn shl(self, rhs: Int32) -> u8 {
        self << rhs.0
    }
}

impl ShlAssign<Int32> for u8 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Int32) {
        *self <<= rhs.0
    }
}

impl Shr<Int32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn shr(self, rhs: Int32) -> Int32 {
        Int32( self.0 >> rhs.0 )
    }
}

impl ShrAssign<Int32> for Int32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Int32) {
        self.0 >>= rhs.0
    }
}

impl Shr<u64> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn shr(self, rhs: u64) -> Int32 {
        Int32( self.0 >> rhs )
    }
}

impl ShrAssign<u64> for Int32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u64) {
        self.0 >>= rhs
    }
}

impl Shr<Int32> for u64 {
    type Output = u64;

    #[inline(always)]
    fn shr(self, rhs: Int32) -> u64 {
        self >> rhs.0
    }
}

impl ShrAssign<Int32> for u64 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Int32) {
        *self >>= rhs.0
    }
}

impl Shr<i32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn shr(self, rhs: i32) -> Int32 {
        Int32( self.0 >> rhs )
    }
}

impl ShrAssign<i32> for Int32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: i32) {
        self.0 >>= rhs
    }
}

impl Shr<Int32> for i32 {
    type Output = i32;

    #[inline(always)]
    fn shr(self, rhs: Int32) -> i32 {
        self >> rhs.0
    }
}

impl ShrAssign<Int32> for i32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Int32) {
        *self >>= rhs.0
    }
}

impl Shr<usize> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn shr(self, rhs: usize) -> Int32 {
        Int32( self.0 >> rhs )
    }
}

impl ShrAssign<usize> for Int32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs
    }
}

impl Shr<Int32> for usize {
    type Output = usize;

    #[inline(always)]
    fn shr(self, rhs: Int32) -> usize {
        self >> rhs.0
    }
}

impl ShrAssign<Int32> for usize {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Int32) {
        *self >>= rhs.0
    }
}

impl Shr<u16> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn shr(self, rhs: u16) -> Int32 {
        Int32( self.0 >> rhs )
    }
}

impl ShrAssign<u16> for Int32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u16) {
        self.0 >>= rhs
    }
}

impl Shr<Int32> for u16 {
    type Output = u16;

    #[inline(always)]
    fn shr(self, rhs: Int32) -> u16 {
        self >> rhs.0
    }
}

impl ShrAssign<Int32> for u16 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Int32) {
        *self >>= rhs.0
    }
}

impl Shr<u8> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn shr(self, rhs: u8) -> Int32 {
        Int32( self.0 >> rhs )
    }
}

impl ShrAssign<u8> for Int32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs
    }
}

impl Shr<Int32> for u8 {
    type Output = u8;

    #[inline(always)]
    fn shr(self, rhs: Int32) -> u8 {
        self >> rhs.0
    }
}

impl ShrAssign<Int32> for u8 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Int32) {
        *self >>= rhs.0
    }
}



#[inline(never)]
fn quotient(a: i32, b: i32) -> i32 {
    let sdividend = unsafe { &mut *(0xD0000068 as *mut SIORegister<i32>) };
    let sdivisor  = unsafe { &mut *(0xD000006C as *mut SIORegister<i32>) };

    let quotient  = unsafe { & *(0xD0000070 as *const SIORegister<i32>) };
    let remainder = unsafe { & *(0xD0000074 as *const SIORegister<i32>) };

    sdividend.write(a);
    sdivisor.write(b);

    delay();

    remainder.read();
    quotient.read()
}

impl Div<Int32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn div(self, rhs: Int32) -> Int32 {
        Int32( quotient(self.0, rhs.0) )
    }
}

impl DivAssign<Int32> for Int32 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Int32) {
        self.0 = quotient(self.0, rhs.0);
    }
}

impl Div<i32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn div(self, rhs: i32) -> Int32 {
        Int32( quotient(self.0, rhs) )
    }
}

impl DivAssign<i32> for Int32 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: i32) {
        self.0 = quotient(self.0, rhs);
    }
}

impl Div<Int32> for i32 {
    type Output = Int32;

    #[inline(always)]
    fn div(self, rhs: Int32) -> Int32 {
        Int32( quotient(self, rhs.0) )
    }
}

impl DivAssign<Int32> for i32 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Int32) {
        *self = quotient(*self, rhs.0);
    }
}



#[inline(never)]
fn remainder(a: i32, b: i32) -> i32 {
    let sdividend = unsafe { &mut *(0xD0000068 as *mut SIORegister<i32>) };
    let sdivisor  = unsafe { &mut *(0xD000006C as *mut SIORegister<i32>) };

    let quotient  = unsafe { & *(0xD0000070 as *const SIORegister<i32>) };
    let remainder = unsafe { & *(0xD0000074 as *const SIORegister<i32>) };

    sdividend.write(a);
    sdivisor.write(b);

    delay();

    let rem = remainder.read();
    quotient.read();

    rem
}

impl Rem<Int32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn rem(self, rhs: Int32) -> Int32 {
        Int32( remainder(self.0, rhs.0) )
    }
}

impl RemAssign<Int32> for Int32 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Int32) {
        self.0 = remainder(self.0, rhs.0);
    }
}

impl Rem<i32> for Int32 {
    type Output = Int32;

    #[inline(always)]
    fn rem(self, rhs: i32) -> Int32 {
        Int32( remainder(self.0, rhs) )
    }
}

impl RemAssign<i32> for Int32 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: i32) {
        self.0 = remainder(self.0, rhs);
    }
}

impl Rem<Int32> for i32 {
    type Output = Int32;

    #[inline(always)]
    fn rem(self, rhs: Int32) -> Int32 {
        Int32( remainder(self, rhs.0) )
    }
}

impl RemAssign<Int32> for i32 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Int32) {
        *self = remainder(*self, rhs.0);
    }
}



impl PartialEq<i32> for Int32 {
    #[inline(always)]
    fn eq(&self, other: &i32) -> bool {
        self.0.eq(other)
    }

    #[inline(always)]
    fn ne(&self, other: &i32) -> bool {
        self.0.ne(other)
    }
}

impl PartialEq<Int32> for Int32 {
    #[inline(always)]
    fn eq(&self, other: &Int32) -> bool {
        self.0.eq(&other.0)
    }

    #[inline(always)]
    fn ne(&self, other: &Int32) -> bool {
        self.0.ne(&other.0)
    }
}

impl Eq for Int32 {}

impl PartialOrd<i32> for Int32 {
    #[inline(always)]
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }

    #[inline(always)]
    fn lt(&self, other: &i32) -> bool {
        self.0.lt(other)
    }

    #[inline(always)]
    fn le(&self, other: &i32) -> bool {
        self.0.le(other)
    }

    #[inline(always)]
    fn gt(&self, other: &i32) -> bool {
        self.0.gt(other)
    }

    #[inline(always)]
    fn ge(&self, other: &i32) -> bool {
        self.0.ge(other)
    }
}

impl PartialOrd<Int32> for Int32 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Int32) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }

    #[inline(always)]
    fn lt(&self, other: &Int32) -> bool {
        self.0.lt(&other.0)
    }

    #[inline(always)]
    fn le(&self, other: &Int32) -> bool {
        self.0.le(&other.0)
    }

    #[inline(always)]
    fn gt(&self, other: &Int32) -> bool {
        self.0.gt(&other.0)
    }

    #[inline(always)]
    fn ge(&self, other: &Int32) -> bool {
        self.0.ge(&other.0)
    }
}

impl Ord for Int32 {
    #[inline(always)]
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
