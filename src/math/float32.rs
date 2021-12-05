//! Float 32 wrapper to allow using the ROM functions.


#[cfg(feature = "bootrom-V2")]
use core::cmp::*;

use core::convert::*;
use core::ops::*;


use crate::math::SFloatTable;

#[cfg(feature = "bootrom-V2")]
use crate::math::Float64;



#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Float32(f32);

impl Float32 {
    /// No op transformation of f32 into Float32.
    #[inline(always)]
    pub const fn new(f: f32) -> Float32 {
        Float32(f)
    }

    /// Computes the sqaure root of the float.
    pub fn sqrt(self) -> Float32 {
        Float32::new( SFloatTable::sqrt(self.0) )
    }

    /// Computes the cosine of the angle in radians (range -128 to 128).
    pub fn cos(self) -> Float32 {
        Float32::new( SFloatTable::cos(self.0) )
    }

    /// Computes the sine of the angle in radians (range -128 to 128).
    pub fn sin(self) -> Float32 {
        Float32::new( SFloatTable::sin(self.0) )
    }

    /// Computes the tangent of the angle in radians (range -128 to 128).
    pub fn tan(self) -> Float32 {
        Float32::new( SFloatTable::tan(self.0) )
    }

    /// Computes the exponent of the float.
    pub fn exp(self) -> Float32 {
        Float32::new( SFloatTable::exp(self.0) )
    }

    /// Computes the natural logarithm of the float.
    pub fn ln(self) -> Float32 {
        Float32::new( SFloatTable::cos(self.0) )
    }

    #[cfg(feature = "bootrom-V2")]
    #[doc = "Computes the arc tangent of X / Y"]
    pub fn atan2<F>(x: F, y: F) -> Float32 where f32: From<F> {
        Float32::new( SFloatTable::atan2(f32::from(x), f32::from(y)) )
    }
}


impl<F> Add<F> for Float32 where f32: From<F> {
    type Output = Float32;

    fn add(self, f: F) -> Float32 {
        Float32::new( SFloatTable::add(self.0, f32::from(f)) )
    }
}

impl Add<Float32> for f32 {
    type Output = Float32;

    fn add(self, f: Float32) -> Float32 {
        Float32::new( SFloatTable::add(self, f.0) )
    }
}

impl<F> Sub<F> for Float32 where f32: From<F> {
    type Output = Float32;

    fn sub(self, f: F) -> Float32 {
        Float32::new( SFloatTable::sub(self.0, f32::from(f)) )
    }
}

impl Sub<Float32> for f32 {
    type Output = Float32;

    fn sub(self, f: Float32) -> Float32 {
        Float32::new( SFloatTable::sub(self, f.0) )
    }
}

impl<F> Mul<F> for Float32 where f32: From<F> {
    type Output = Float32;

    fn mul(self, f: F) -> Float32 {
        Float32::new( SFloatTable::mul(self.0, f32::from(f)) )
    }
}

impl Mul<Float32> for f32 {
    type Output = Float32;

    fn mul(self, f: Float32) -> Float32 {
        Float32::new( SFloatTable::mul(self, f.0) )
    }
}

impl<F> Div<F> for Float32 where f32: From<F> {
    type Output = Float32;

    fn div(self, f: F) -> Float32 {
        Float32::new( SFloatTable::div(self.0, f32::from(f)) )
    }
}

impl Div<Float32> for f32 {
    type Output = Float32;

    fn div(self, f: Float32) -> Float32 {
        Float32::new( SFloatTable::div(self, f.0) )
    }
}


impl From<Float32> for i8 {
    fn from(f: Float32) -> i8 {
        SFloatTable::float2int(f.0) as i8
    }
}

impl From<Float32> for u8 {
    fn from(f: Float32) -> u8 {
        SFloatTable::float2uint(f.0) as u8
    }
}

impl From<Float32> for i16 {
    fn from(f: Float32) -> i16 {
        SFloatTable::float2int(f.0) as i16
    }
}

impl From<Float32> for u16 {
    fn from(f: Float32) -> u16 {
        SFloatTable::float2uint(f.0) as u16
    }
}

impl From<Float32> for i32 {
    fn from(f: Float32) -> i32 {
        SFloatTable::float2int(f.0)
    }
}

impl From<Float32> for u32 {
    fn from(f: Float32) -> u32 {
        SFloatTable::float2uint(f.0)
    }
}

#[cfg(feature = "bootrom-V2")]
impl From<Float32> for i64 {
    fn from(f: Float32) -> i64 {
        SFloatTable::float2int64(f.0)
    }
}

#[cfg(feature = "bootrom-V2")]
impl From<Float32> for u64 {
    fn from(f: Float32) -> u64 {
        SFloatTable::float2uint64(f.0)
    }
}

impl From<i8> for Float32 {
    fn from(i: i8) -> Float32 {
        Float32::new( SFloatTable::int2float( i as i32 ) )
    }
}

impl From<u8> for Float32 {
    fn from(u: u8) -> Float32 {
        Float32::new( SFloatTable::uint2float( u as u32 ) )
    }
}

impl From<i16> for Float32 {
    fn from(i: i16) -> Float32 {
        Float32::new( SFloatTable::int2float( i as i32 ) )
    }
}

impl From<u16> for Float32 {
    fn from(u: u16) -> Float32 {
        Float32::new( SFloatTable::uint2float( u as u32 ) )
    }
}

impl From<i32> for Float32 {
    fn from(i: i32) -> Float32 {
        Float32::new( SFloatTable::int2float( i ) )
    }
}

impl From<u32> for Float32 {
    fn from(u: u32) -> Float32 {
        Float32::new( SFloatTable::uint2float( u ) )
    }
}

#[cfg(feature = "bootrom-V2")]
impl From<i64> for Float32 {
    fn from(i: i64) -> Float32 {
        Float32::new( SFloatTable::int642float( i ) )
    }
}

#[cfg(feature = "bootrom-V2")]
impl From<u64> for Float32 {
    fn from(u: u64) -> Float32 {
        Float32::new( SFloatTable::uint642float( u ) )
    }
}

impl From<f32> for Float32 {
    #[inline(always)]
    fn from(f: f32) -> Float32 {
        Float32( f )
    }
}

impl From<Float32> for f32 {
    #[inline(always)]
    fn from(f: Float32) -> f32 {
        f.0
    }
}

#[cfg(feature = "bootrom-V2")]
impl From<Float32> for Float64 {
    fn from(f: Float32) -> Float64 {
        SFloatTable::float2double(f)
    }
}


#[cfg(feature = "bootrom-V2")]
impl Eq for Float32 {}

#[cfg(feature = "bootrom-V2")]
impl<X> PartialEq<X> for Float32 where Float32: From<X> {
    fn eq(&self, other: &X) -> bool {
        let other = Float32::from(*other);

        SFloatTable::cmp(other) == 0
    }

    fn ne(&self, other: &X) -> bool {
        let other = Float32::from(*other);

        SFloatTable::cmp(other) != 0
    }
}



#[cfg(feature = "bootrom-V2")]
impl Ord for Float32 {
    fn cmp(&self, other: &Float32) -> Ordering {
        let other = Float32::from(*other);

        match SFloatTable::cmp(other) {
            -1 => Ordering::Less,
             0 => Ordering::Equal,
             1 => Ordering::Greater,

            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "bootrom-V2")]
impl<X> PartialOrd<X> for Float32 where Float32: From<X> {
    fn partial_cmp(&self, other: &X) -> Option<Ordering> {
        let other = Float32::from(*other);

        match SFloatTable::cmp(other) {
            -1 => Some(Ordering::Less),
             0 => Some(Ordering::Equal),
             1 => Some(Ordering::Greater),

            _ => unreachable!(),
        }
    }

    fn lt(&self, other: &X) -> bool {
        let other = Float32::from(*other);

        SFloatTable::cmp(other) == -1
    }

    fn le(&self, other: &X) -> bool {
        let other = Float32::from(*other);

        SFloatTable::cmp(other) != 1
    }

    fn gt(&self, other: &X) -> bool {
        let other = Float32::from(*other);

        SFloatTable::cmp(other) == 1
    }

    fn ge(&self, other: &X) -> bool {
        let other = Float32::from(*other);

        SFloatTable::cmp(other) != -1
    }
}