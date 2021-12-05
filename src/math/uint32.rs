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

impl Add<UInt32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn add(self, rhs: UInt32) -> UInt32 {
        UInt32( self.0 + rhs.0 )
    }
}

impl AddAssign<UInt32> for UInt32{
    #[inline(always)]
    fn add_assign(&mut self, rhs: UInt32) {
        self.0 += rhs.0
    }
}

impl Add<u32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn add(self, rhs: u32) -> UInt32 {
        UInt32( self.0 + rhs )
    }
}

impl AddAssign<u32> for UInt32{
    #[inline(always)]
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs
    }
}

impl Add<UInt32> for u32 {
    type Output = UInt32;

    #[inline(always)]
    fn add(self, rhs: UInt32) -> UInt32 {
        UInt32( self + rhs.0 )
    }
}

impl AddAssign<UInt32> for u32{
    #[inline(always)]
    fn add_assign(&mut self, rhs: UInt32) {
        *self += rhs.0
    }
}

impl Sub<UInt32> for UInt32{
    type Output = UInt32;

    #[inline(always)]
    fn sub(self, rhs: UInt32) -> UInt32 {
        UInt32( self.0 - rhs.0 )
    }
}

impl SubAssign<UInt32> for UInt32{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: UInt32) {
        self.0 -= rhs.0
    }
}

impl Sub<u32> for UInt32{
    type Output = UInt32;

    #[inline(always)]
    fn sub(self, rhs: u32) -> UInt32 {
        UInt32( self.0 - rhs )
    }
}

impl SubAssign<u32> for UInt32{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: u32) {
        self.0 -= rhs
    }
}

impl Sub<UInt32> for u32{
    type Output = UInt32;

    #[inline(always)]
    fn sub(self, rhs: UInt32) -> UInt32 {
        UInt32( self - rhs.0 )
    }
}

impl SubAssign<UInt32> for u32{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: UInt32) {
        *self -= rhs.0
    }
}

impl Mul<UInt32> for UInt32{
    type Output = UInt32;

    #[inline(always)]
    fn mul(self, rhs: UInt32) -> UInt32 {
        UInt32( self.0 * rhs.0 )
    }
}

impl MulAssign<UInt32> for UInt32 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: UInt32) {
        self.0 *= rhs.0
    }
}

impl Mul<u32> for UInt32{
    type Output = UInt32;

    #[inline(always)]
    fn mul(self, rhs: u32) -> UInt32 {
        UInt32( self.0 * rhs )
    }
}

impl MulAssign<u32> for UInt32 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: u32) {
        self.0 *= rhs
    }
}

impl Mul<UInt32> for u32{
    type Output = UInt32;

    #[inline(always)]
    fn mul(self, rhs: UInt32) -> UInt32 {
        UInt32( self * rhs.0 )
    }
}

impl MulAssign<UInt32> for u32 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: UInt32) {
        *self *= rhs.0
    }
}

impl BitAnd<UInt32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn bitand(self, rhs: UInt32) -> UInt32 {
        UInt32( self.0 & rhs.0 )
    }
}

impl BitAndAssign<UInt32> for UInt32 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: UInt32) {
        self.0 &= rhs.0
    }
}

impl BitAnd<u32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn bitand(self, rhs: u32) -> UInt32 {
        UInt32( self.0 & rhs )
    }
}

impl BitAndAssign<u32> for UInt32 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: u32) {
        self.0 &= rhs
    }
}

impl BitAnd<UInt32> for u32 {
    type Output = UInt32;

    #[inline(always)]
    fn bitand(self, rhs: UInt32) -> UInt32 {
        UInt32( self & rhs.0 )
    }
}

impl BitAndAssign<UInt32> for u32 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: UInt32) {
        *self &= rhs.0
    }
}

impl BitOr<UInt32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn bitor(self, rhs: UInt32) -> UInt32 {
        UInt32( self.0 | rhs.0 )
    }
}

impl BitOrAssign<UInt32> for UInt32 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: UInt32) {
        self.0 |= rhs.0
    }
}

impl BitOr<u32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn bitor(self, rhs: u32) -> UInt32 {
        UInt32( self.0 | rhs )
    }
}

impl BitOrAssign<u32> for UInt32 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: u32) {
        self.0 |= rhs
    }
}

impl BitOr<UInt32> for u32 {
    type Output = UInt32;

    #[inline(always)]
    fn bitor(self, rhs: UInt32) -> UInt32 {
        UInt32( self | rhs.0 )
    }
}

impl BitOrAssign<UInt32> for u32 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: UInt32) {
        *self |= rhs.0
    }
}

impl BitXor<UInt32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn bitxor(self, rhs: UInt32) -> UInt32 {
        UInt32( self.0 ^ rhs.0 )
    }
}

impl BitXorAssign<UInt32> for UInt32 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: UInt32) {
        self.0 ^= rhs.0
    }
}

impl BitXor<u32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn bitxor(self, rhs: u32) -> UInt32 {
        UInt32( self.0 ^ rhs )
    }
}

impl BitXorAssign<u32> for UInt32 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: u32) {
        self.0 ^= rhs
    }
}

impl BitXor<UInt32> for u32 {
    type Output = UInt32;

    #[inline(always)]
    fn bitxor(self, rhs: UInt32) -> UInt32 {
        UInt32( self ^ rhs.0 )
    }
}

impl BitXorAssign<UInt32> for u32 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: UInt32) {
        *self ^= rhs.0
    }
}

impl Not for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn not(self) -> UInt32 {
        UInt32( !self.0 )
    }
}

impl Shl<UInt32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn shl(self, rhs: UInt32) -> UInt32 {
        UInt32( self.0 << rhs.0 )
    }
}

impl ShlAssign<UInt32> for UInt32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: UInt32) {
        self.0 <<= rhs.0
    }
}

impl Shl<u64> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn shl(self, rhs: u64) -> UInt32 {
        UInt32( self.0 << rhs )
    }
}

impl ShlAssign<u64> for UInt32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u64) {
        self.0 <<= rhs
    }
}

impl Shl<UInt32> for u64 {
    type Output = u64;

    #[inline(always)]
    fn shl(self, rhs: UInt32) -> u64 {
        self << rhs.0
    }
}

impl ShlAssign<UInt32> for u64 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: UInt32) {
        *self <<= rhs.0
    }
}

impl Shl<u32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn shl(self, rhs: u32) -> UInt32 {
        UInt32( self.0 << rhs )
    }
}

impl ShlAssign<u32> for UInt32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u32) {
        self.0 <<= rhs
    }
}

impl Shl<UInt32> for u32 {
    type Output = u32;

    #[inline(always)]
    fn shl(self, rhs: UInt32) -> u32 {
        self << rhs.0
    }
}

impl ShlAssign<UInt32> for u32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: UInt32) {
        *self <<= rhs.0
    }
}

impl Shl<usize> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn shl(self, rhs: usize) -> UInt32 {
        UInt32( self.0 << rhs )
    }
}

impl ShlAssign<usize> for UInt32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs
    }
}

impl Shl<UInt32> for usize {
    type Output = usize;

    #[inline(always)]
    fn shl(self, rhs: UInt32) -> usize {
        self << rhs.0
    }
}

impl ShlAssign<UInt32> for usize {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: UInt32) {
        *self <<= rhs.0
    }
}

impl Shl<u16> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn shl(self, rhs: u16) -> UInt32 {
        UInt32( self.0 << rhs )
    }
}

impl ShlAssign<u16> for UInt32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u16) {
        self.0 <<= rhs
    }
}

impl Shl<UInt32> for u16 {
    type Output = u16;

    #[inline(always)]
    fn shl(self, rhs: UInt32) -> u16 {
        self << rhs.0
    }
}

impl ShlAssign<UInt32> for u16 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: UInt32) {
        *self <<= rhs.0
    }
}

impl Shl<u8> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn shl(self, rhs: u8) -> UInt32 {
        UInt32( self.0 << rhs )
    }
}

impl ShlAssign<u8> for UInt32 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs
    }
}

impl Shl<UInt32> for u8 {
    type Output = u8;

    #[inline(always)]
    fn shl(self, rhs: UInt32) -> u8 {
        self << rhs.0
    }
}

impl ShlAssign<UInt32> for u8 {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: UInt32) {
        *self <<= rhs.0
    }
}

impl Shr<UInt32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn shr(self, rhs: UInt32) -> UInt32 {
        UInt32( self.0 >> rhs.0 )
    }
}

impl ShrAssign<UInt32> for UInt32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: UInt32) {
        self.0 >>= rhs.0
    }
}

impl Shr<u64> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn shr(self, rhs: u64) -> UInt32 {
        UInt32( self.0 >> rhs )
    }
}

impl ShrAssign<u64> for UInt32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u64) {
        self.0 >>= rhs
    }
}

impl Shr<UInt32> for u64 {
    type Output = u64;

    #[inline(always)]
    fn shr(self, rhs: UInt32) -> u64 {
        self >> rhs.0
    }
}

impl ShrAssign<UInt32> for u64 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: UInt32) {
        *self >>= rhs.0
    }
}

impl Shr<u32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn shr(self, rhs: u32) -> UInt32 {
        UInt32( self.0 >> rhs )
    }
}

impl ShrAssign<u32> for UInt32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u32) {
        self.0 >>= rhs
    }
}

impl Shr<UInt32> for u32 {
    type Output = u32;

    #[inline(always)]
    fn shr(self, rhs: UInt32) -> u32 {
        self >> rhs.0
    }
}

impl ShrAssign<UInt32> for u32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: UInt32) {
        *self >>= rhs.0
    }
}

impl Shr<usize> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn shr(self, rhs: usize) -> UInt32 {
        UInt32( self.0 >> rhs )
    }
}

impl ShrAssign<usize> for UInt32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: usize) {
        self.0 >>= rhs
    }
}

impl Shr<UInt32> for usize {
    type Output = usize;

    #[inline(always)]
    fn shr(self, rhs: UInt32) -> usize {
        self >> rhs.0
    }
}

impl ShrAssign<UInt32> for usize {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: UInt32) {
        *self >>= rhs.0
    }
}

impl Shr<u16> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn shr(self, rhs: u16) -> UInt32 {
        UInt32( self.0 >> rhs )
    }
}

impl ShrAssign<u16> for UInt32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u16) {
        self.0 >>= rhs
    }
}

impl Shr<UInt32> for u16 {
    type Output = u16;

    #[inline(always)]
    fn shr(self, rhs: UInt32) -> u16 {
        self >> rhs.0
    }
}

impl ShrAssign<UInt32> for u16 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: UInt32) {
        *self >>= rhs.0
    }
}

impl Shr<u8> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn shr(self, rhs: u8) -> UInt32 {
        UInt32( self.0 >> rhs )
    }
}

impl ShrAssign<u8> for UInt32 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs
    }
}

impl Shr<UInt32> for u8 {
    type Output = u8;

    #[inline(always)]
    fn shr(self, rhs: UInt32) -> u8 {
        self >> rhs.0
    }
}

impl ShrAssign<UInt32> for u8 {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: UInt32) {
        *self >>= rhs.0
    }
}



#[inline(never)]
fn quotient(a: u32, b: u32) -> u32 {
    let sdividend = unsafe { &mut *(0xD0000068 as *mut SIORegister<u32>) };
    let sdivisor  = unsafe { &mut *(0xD000006C as *mut SIORegister<u32>) };

    let quotient  = unsafe { & *(0xD0000070 as *const SIORegister<u32>) };
    let remainder = unsafe { & *(0xD0000074 as *const SIORegister<u32>) };

    sdividend.write(a);
    sdivisor.write(b);

    delay();

    remainder.read();
    quotient.read()
}

impl Div<UInt32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn div(self, rhs: UInt32) -> UInt32 {
        UInt32( quotient(self.0, rhs.0) )
    }
}

impl DivAssign<UInt32> for UInt32 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: UInt32) {
        self.0 = quotient(self.0, rhs.0);
    }
}

impl Div<u32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn div(self, rhs: u32) -> UInt32 {
        UInt32( quotient(self.0, rhs) )
    }
}

impl DivAssign<u32> for UInt32 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: u32) {
        self.0 = quotient(self.0, rhs);
    }
}

impl Div<UInt32> for u32 {
    type Output = UInt32;

    #[inline(always)]
    fn div(self, rhs: UInt32) -> UInt32 {
        UInt32( quotient(self, rhs.0) )
    }
}

impl DivAssign<UInt32> for u32 {
    #[inline(always)]
    fn div_assign(&mut self, rhs: UInt32) {
        *self = quotient(*self, rhs.0);
    }
}



#[inline(never)]
fn remainder(a: u32, b: u32) -> u32 {
    let sdividend = unsafe { &mut *(0xD0000068 as *mut SIORegister<u32>) };
    let sdivisor  = unsafe { &mut *(0xD000006C as *mut SIORegister<u32>) };

    let quotient  = unsafe { & *(0xD0000070 as *const SIORegister<u32>) };
    let remainder = unsafe { & *(0xD0000074 as *const SIORegister<u32>) };

    sdividend.write(a);
    sdivisor.write(b);

    delay();

    let rem = remainder.read();
    quotient.read();

    rem
}

impl Rem<UInt32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn rem(self, rhs: UInt32) -> UInt32 {
        UInt32( remainder(self.0, rhs.0) )
    }
}

impl RemAssign<UInt32> for UInt32 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: UInt32) {
        self.0 = remainder(self.0, rhs.0);
    }
}

impl Rem<u32> for UInt32 {
    type Output = UInt32;

    #[inline(always)]
    fn rem(self, rhs: u32) -> UInt32 {
        UInt32( remainder(self.0, rhs) )
    }
}

impl RemAssign<u32> for UInt32 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: u32) {
        self.0 = remainder(self.0, rhs);
    }
}

impl Rem<UInt32> for u32 {
    type Output = UInt32;

    #[inline(always)]
    fn rem(self, rhs: UInt32) -> UInt32 {
        UInt32( remainder(self, rhs.0) )
    }
}

impl RemAssign<UInt32> for u32 {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: UInt32) {
        *self = remainder(*self, rhs.0);
    }
}



impl PartialEq<u32> for UInt32 {
    #[inline(always)]
    fn eq(&self, other: &u32) -> bool {
        self.0.eq(other)
    }

    #[inline(always)]
    fn ne(&self, other: &u32) -> bool {
        self.0.ne(other)
    }
}

impl PartialEq<UInt32> for UInt32 {
    #[inline(always)]
    fn eq(&self, other: &UInt32) -> bool {
        self.0.eq(&other.0)
    }

    #[inline(always)]
    fn ne(&self, other: &UInt32) -> bool {
        self.0.ne(&other.0)
    }
}

impl Eq for UInt32 {}

impl PartialOrd<u32> for UInt32 {
    #[inline(always)]
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }

    #[inline(always)]
    fn lt(&self, other: &u32) -> bool {
        self.0.lt(other)
    }

    #[inline(always)]
    fn le(&self, other: &u32) -> bool {
        self.0.le(other)
    }

    #[inline(always)]
    fn gt(&self, other: &u32) -> bool {
        self.0.gt(other)
    }

    #[inline(always)]
    fn ge(&self, other: &u32) -> bool {
        self.0.ge(other)
    }
}

impl PartialOrd<UInt32> for UInt32 {
    #[inline(always)]
    fn partial_cmp(&self, other: &UInt32) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }

    #[inline(always)]
    fn lt(&self, other: &UInt32) -> bool {
        self.0.lt(&other.0)
    }

    #[inline(always)]
    fn le(&self, other: &UInt32) -> bool {
        self.0.le(&other.0)
    }

    #[inline(always)]
    fn gt(&self, other: &UInt32) -> bool {
        self.0.gt(&other.0)
    }

    #[inline(always)]
    fn ge(&self, other: &UInt32) -> bool {
        self.0.ge(&other.0)
    }
}

impl Ord for UInt32 {
    #[inline(always)]
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
