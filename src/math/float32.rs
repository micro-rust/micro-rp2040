//! Float 32 wrapper to allow using the ROM functions.


#[cfg(feature = "bootrom-V2")]
use core::cmp::*;

use core::convert::*;
use core::ops::*;


use crate::math::SFTABLE;

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
        Float32::new( SFTABLE.sqrt(self.0) )
    }

    /// Computes the cosine of the angle in radians (range -128 to 128).
    pub fn cos(self) -> Float32 {
        Float32::new( SFTABLE.cos(self.0) )
    }

    /// Computes the sine of the angle in radians (range -128 to 128).
    pub fn sin(self) -> Float32 {
        Float32::new( SFTABLE.sin(self.0) )
    }

    /// Computes the tangent of the angle in radians (range -128 to 128).
    pub fn tan(self) -> Float32 {
        Float32::new( SFTABLE.tan(self.0) )
    }

    /// Computes the exponent of the float.
    pub fn exp(self) -> Float32 {
        Float32::new( SFTABLE.exp(self.0) )
    }

    /// Computes the natural logarithm of the float.
    pub fn ln(self) -> Float32 {
        Float32::new( SFTABLE.cos(self.0) )
    }

    #[cfg(feature = "bootrom-V2")]
    #[doc = "Computes the arc tangent of X / Y"]
    pub fn atan2<F>(x: F, y: F) -> Float32 where f32: From<F> {
        Float32::new( SFTABLE.atan2(f32::from(x), f32::from(y)) )
    }
}


impl<F> Add<F> for Float32 where f32: From<F> {
    type Output = Float32;

    fn add(self, b: F) -> Float32 {
        Float32::new( SFTABLE.add(self.0, f32::from(b)) )
    }
}

impl<F> Sub<F> for Float32 where f32: From<F> {
    type Output = Float32;

    fn sub(self, b: F) -> Float32 {
        Float32::new( SFTABLE.sub(self.0, f32::from(b)) )
    }
}

impl<F> Mul<F> for Float32 where f32: From<F> {
    type Output = Float32;

    fn mul(self, b: F) -> Float32 {
        Float32::new( SFTABLE.mul(self.0, f32::from(b)) )
    }
}

impl<F> Div<F> for Float32 where f32: From<F> {
    type Output = Float32;

    fn div(self, b: F) -> Float32 {
        Float32::new( SFTABLE.div(self.0, f32::from(b)) )
    }
}

impl From<Float32> for i32 {
    fn from(f: Float32) -> i32 {
        SFTABLE.float2int(f.0)
    }
}

impl From<Float32> for u32 {
    fn from(f: Float32) -> u32 {
        SFTABLE.float2uint(f.0)
    }
}

#[cfg(feature = "bootrom-V2")]
impl From<Float32> for i64 {
    fn from(f: Float32) -> i64 {
        SFTABLE.float2int64(f.0)
    }
}

#[cfg(feature = "bootrom-V2")]
impl From<Float32> for u64 {
    fn from(f: Float32) -> u64 {
        SFTABLE.float2uint64(f.0)
    }
}

impl From<i32> for Float32 {
    fn from(i: i32) -> Float32 {
        Float32::new( SFTABLE.int2float( i ) )
    }
}

impl From<u32> for Float32 {
    fn from(u: u32) -> Float32 {
        Float32::new( SFTABLE.uint2float( u ) )
    }
}

#[cfg(feature = "bootrom-V2")]
impl From<i64> for Float32 {
    fn from(i: i64) -> Float32 {
        Float32::new( SFTABLE.int642float( i ) )
    }
}

#[cfg(feature = "bootrom-V2")]
impl From<u64> for Float32 {
    fn from(u: u64) -> Float32 {
        Float32::new( SFTABLE.uint642float( u ) )
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
        SFTABLE.float2double(f)
    }
}


#[cfg(feature = "bootrom-V2")]
impl Eq for Float32 {}

#[cfg(feature = "bootrom-V2")]
impl<X> PartialEq<X> for Float32 where Float32: From<X> {
    fn eq(&self, other: &X) -> bool {
        let other = Float32::from(*other);

        SFTABLE.cmp(other) == 0
    }

    fn ne(&self, other: &X) -> bool {
        let other = Float32::from(*other);

        SFTABLE.cmp(other) != 0
    }
}



#[cfg(feature = "bootrom-V2")]
impl Ord for Float32 {
    fn cmp(&self, other: &Float32) -> Ordering {
        let other = Float32::from(*other);

        match SFTABLE.cmp(other) {
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

        match SFTABLE.cmp(other) {
            -1 => Some(Ordering::Less),
             0 => Some(Ordering::Equal),
             1 => Some(Ordering::Greater),

            _ => unreachable!(),
        }
    }

    fn lt(&self, other: &X) -> bool {
        let other = Float32::from(*other);

        SFTABLE.cmp(other) == -1
    }

    fn le(&self, other: &X) -> bool {
        let other = Float32::from(*other);

        SFTABLE.cmp(other) != 1
    }

    fn gt(&self, other: &X) -> bool {
        let other = Float32::from(*other);

        SFTABLE.cmp(other) == 1
    }

    fn ge(&self, other: &X) -> bool {
        let other = Float32::from(*other);

        SFTABLE.cmp(other) != -1
    }
}