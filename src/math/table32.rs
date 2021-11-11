//! Abstraction of the ROM single float table.



use core::mem::transmute;
use core::ptr::read_volatile as read;



#[cfg(not(feature = "bootrom-V2"))]
const SFTSIZE: usize = 21;

#[cfg(feature = "bootrom-V2")]
const SFTSIZE: usize = 32;



/// Force on each pointer read a volatile.
/// If we don't do this, the compiler assumes all entries are NULL and 
/// sets all accesses to a panic.
#[repr(C)]
pub struct SFloatTable(pub(crate) [u32; SFTSIZE]);

impl SFloatTable {
    pub const fn empty() -> SFloatTable {
        SFloatTable([0u32; SFTSIZE])
    }

    #[inline(always)]
    pub(super) fn add(&self, a: f32, b: f32) -> f32 {
        let f: extern "C" fn(f32, f32) -> f32 = unsafe { transmute( read((&self.0[0]) as *const u32) ) };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn sub(&self, a: f32, b: f32) -> f32 {
        let f: extern "C" fn(f32, f32) -> f32 = unsafe { transmute( read((&self.0[1]) as *const u32) ) };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn mul(&self, a: f32, b: f32) -> f32 {
        let f: extern "C" fn(f32, f32) -> f32 = unsafe { transmute( read((&self.0[2]) as *const u32) ) };

        f(a, b)
    }

    #[inline(never)]
    pub(super) fn div(&self, a: f32, b: f32) -> f32 {
        let f: extern "C" fn(f32, f32) -> f32 = unsafe { transmute( read((&self.0[3]) as *const u32) ) };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn sqrt(&self, a: f32) -> f32 {
        let f: extern "C" fn(f32) -> f32 = unsafe { transmute( read((&self.0[6]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn float2int(&self, a: f32) -> i32 {
        let f: extern "C" fn(f32) -> i32 = unsafe { transmute( read((&self.0[7]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn float2fix(&self, a: f32) -> i32 {
        let f: extern "C" fn(f32) -> i32 = unsafe { transmute( read((&self.0[8]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn float2uint(&self, a: f32) -> u32 {
        let f: extern "C" fn(f32) -> u32 = unsafe { transmute( read((&self.0[9]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn float2ufix(&self, a: f32) -> u32 {
        let f: extern "C" fn(f32) -> u32 = unsafe { transmute( read((&self.0[10]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn int2float(&self, a: i32) -> f32 {
        let f: extern "C" fn(i32) -> f32 = unsafe { transmute( read((&self.0[11]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn fix2float(&self, a: i32) -> f32 {
        let f: extern "C" fn(i32) -> f32 = unsafe { transmute( read((&self.0[12]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn uint2float(&self, a: u32) -> f32 {
        let f: extern "C" fn(u32) -> f32 = unsafe { transmute( read((&self.0[13]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn ufix2float(&self, a: u32) -> f32 {
        let f: extern "C" fn(u32) -> f32 = unsafe { transmute( read((&self.0[14]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn cos(&self, a: f32) -> f32 {
        let f: extern "C" fn(f32) -> f32 = unsafe { transmute( read((&self.0[15]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn sin(&self, a: f32) -> f32 {
        let f: extern "C" fn(f32) -> f32 = unsafe { transmute( read((&self.0[16]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn tan(&self, a: f32) -> f32 {
        let f: extern "C" fn(f32) -> f32 = unsafe { transmute( read((&self.0[17]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn exp(&self, a: f32) -> f32 {
        let f: extern "C" fn(f32) -> f32 = unsafe { transmute( read((&self.0[19]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn ln(&self, a: f32) -> f32 {
        let f: extern "C" fn(f32) -> f32 = unsafe { transmute( read((&self.0[20]) as *const u32) ) };

        f(a)
    }
}

#[cfg(feature = "bootrom-V2")]
impl SFloatTable {
    #[inline(always)]
    pub(super) fn cmp(&self, a: f32, b: f32) -> i32 {
        let f: extern "C" fn(f32, f32) -> i32 = unsafe { transmute( read((&self.0[21]) as *const u32) ) };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn atan2(&self, a: f32, b: f32) -> f32 {
        let f: extern "C" fn(f32, f32) -> f32 = unsafe { transmute( read((&self.0[22]) as *const u32) ) };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn int642float(&self, a: i64) -> f32 {
        let f: extern "C" fn(i64) -> f32 = unsafe { transmute( read((&self.0[23]) as *const u32) ) };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn fix642float(&self, a: i64, b: i32) -> f32 {
        let f: extern "C" fn(i64, i32) -> f32 = unsafe { transmute( read((&self.0[24]) as *const u32) ) };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn uint642float(&self, a: u64) -> f32 {
        let f: extern "C" fn(u64) -> f32 = unsafe { transmute( read((&self.0[25]) as *const u32) ) };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn ufix642float(&self, a: u64, b: i32) -> f32 {
        let f: extern "C" fn(u64, i32) -> f32 = unsafe { transmute( read((&self.0[26]) as *const u32) ) };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn float2int64(&self, a: f32) -> i64 {
        let f: extern "C" fn(f32) -> i64 = unsafe { transmute( read((&self.0[27]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn float2fix64(&self, a: f32, b: i32) -> i64 {
        let f: extern "C" fn(f32, i32) -> i64 = unsafe { transmute( read((&self.0[28]) as *const u32) ) };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn float2uint64(&self, a: f32) -> u64 {
        let f: extern "C" fn(f32) -> u64 = unsafe { transmute( read((&self.0[29]) as *const u32) ) };

        f(a)
    }

    #[inline(always)]
    pub(super) fn float2ufix64(&self, a: f32, b: i32) -> u64 {
        let f: extern "C" fn(f32, i32) -> u64 = unsafe { transmute( read((&self.0[30]) as *const u32) ) };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn float2double(&self, a: f32) -> f64 {
        let f: extern "C" fn(f32) -> f64 = unsafe { transmute( read((&self.0[31]) as *const u32) ) };

        f(a)
    }
}