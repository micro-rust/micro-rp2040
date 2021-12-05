//! Abstraction of the ROM single float table.



use core::mem::transmute;


#[cfg(not(feature = "bootrom-V2"))]
const SFTSIZE: usize = 21;

#[cfg(feature = "bootrom-V2")]
const SFTSIZE: usize = 32;



/// Force on each pointer read a volatile.
/// If we don't do this, the compiler assumes all entries are NULL and 
/// sets all accesses to a panic.
#[repr(C)]
pub struct SFloatTable;

impl SFloatTable {
    /// Loads a pointer to the ROM function table.
    pub(crate) fn load(ptr: u32) {
        unsafe { core::ptr::write_volatile( &mut super::SFTABLE as *mut u32, ptr ); }
    }

    /// Transforms the pointer into a reference.
    #[inline(always)]
    fn ptr() -> &'static [*const u32; SFTSIZE] {
        unsafe { &*(super::SFTABLE as *const [*const u32; SFTSIZE]) }
    }

    #[inline(always)]
    pub(super) fn add(a: f32, b: f32) -> f32 {
        let f: extern "C" fn(f32, f32) -> f32 = unsafe {
            transmute(
                Self::ptr()[0]
            )
        };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn sub(a: f32, b: f32) -> f32 {
        let f: extern "C" fn(f32, f32) -> f32 = unsafe {
            transmute(
                Self::ptr()[1]
            )
        };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn mul(a: f32, b: f32) -> f32 {
        let f: extern "C" fn(f32, f32) -> f32 = unsafe {
            transmute(
                Self::ptr()[2]
            )
        };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn div(a: f32, b: f32) -> f32 {
        let f: extern "C" fn(f32, f32) -> f32 = unsafe {
            transmute(
                Self::ptr()[3]
            )
        };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn sqrt(a: f32) -> f32 {
        let f: extern "C" fn(f32) -> f32 = unsafe {
            transmute(
                Self::ptr()[6]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn float2int(a: f32) -> i32 {
        let f: extern "C" fn(f32) -> i32 = unsafe {
            transmute(
                Self::ptr()[7]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn float2fix(a: f32) -> i32 {
        let f: extern "C" fn(f32) -> i32 = unsafe {
            transmute(
                Self::ptr()[8]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn float2uint(a: f32) -> u32 {
        let f: extern "C" fn(f32) -> u32 = unsafe {
            transmute(
                Self::ptr()[9]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn float2ufix(a: f32) -> u32 {
        let f: extern "C" fn(f32) -> u32 = unsafe {
            transmute(
                Self::ptr()[10]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn int2float(a: i32) -> f32 {
        let f: extern "C" fn(i32) -> f32 = unsafe {
            transmute(
                Self::ptr()[11]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn fix2float(a: i32) -> f32 {
        let f: extern "C" fn(i32) -> f32 = unsafe {
            transmute(
                Self::ptr()[12]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn uint2float(a: u32) -> f32 {
        let f: extern "C" fn(u32) -> f32 = unsafe {
            transmute(
                Self::ptr()[13]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn ufix2float(a: u32) -> f32 {
        let f: extern "C" fn(u32) -> f32 = unsafe {
            transmute(
                Self::ptr()[14]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn cos(a: f32) -> f32 {
        let f: extern "C" fn(f32) -> f32 = unsafe {
            transmute(
                Self::ptr()[15]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn sin(a: f32) -> f32 {
        let f: extern "C" fn(f32) -> f32 = unsafe {
            transmute(
                Self::ptr()[16]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn tan(a: f32) -> f32 {
        let f: extern "C" fn(f32) -> f32 = unsafe {
            transmute(
                Self::ptr()[17]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn exp(a: f32) -> f32 {
        let f: extern "C" fn(f32) -> f32 = unsafe {
            transmute(
                Self::ptr()[19]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn ln(a: f32) -> f32 {
        let f: extern "C" fn(f32) -> f32 = unsafe {
            transmute(
                Self::ptr()[20]
            )
        };

        f(a)
    }
}

#[cfg(feature = "bootrom-V2")]
impl SFloatTable {
    #[inline(always)]
    pub(super) fn cmp(a: f32, b: f32) -> i32 {
        let f: extern "C" fn(f32, f32) -> i32 = unsafe {
            transmute(
                Self::ptr()[21]
            )
        };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn atan2(a: f32, b: f32) -> f32 {
        let f: extern "C" fn(f32, f32) -> f32 = unsafe {
            transmute(
                Self::ptr()[22]
            )
        };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn int642float(a: i64) -> f32 {
        let f: extern "C" fn(i64) -> f32 = unsafe {
            transmute(
                Self::ptr()[23]
            )
        };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn fix642float(a: i64, b: i32) -> f32 {
        let f: extern "C" fn(i64, i32) -> f32 = unsafe {
            transmute(
                Self::ptr()[24]
            )
        };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn uint642float(a: u64) -> f32 {
        let f: extern "C" fn(u64) -> f32 = unsafe {
            transmute(
                Self::ptr()[25]
            )
        };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn ufix642float(a: u64, b: i32) -> f32 {
        let f: extern "C" fn(u64, i32) -> f32 = unsafe {
            transmute(
                Self::ptr()[26]
            )
        };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn float2int64(a: f32) -> i64 {
        let f: extern "C" fn(f32) -> i64 = unsafe {
            transmute(
                Self::ptr()[27]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn float2fix64(a: f32, b: i32) -> i64 {
        let f: extern "C" fn(f32, i32) -> i64 = unsafe {
            transmute(
                Self::ptr()[28]
            )
        };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn float2uint64(a: f32) -> u64 {
        let f: extern "C" fn(f32) -> u64 = unsafe {
            transmute(
                Self::ptr()[29]
            )
        };

        f(a)
    }

    #[inline(always)]
    pub(super) fn float2ufix64(a: f32, b: i32) -> u64 {
        let f: extern "C" fn(f32, i32) -> u64 = unsafe {
            transmute(
                Self::ptr()[30]
            )
        };

        f(a, b)
    }

    #[inline(always)]
    pub(super) fn float2double(a: f32) -> f64 {
        let f: extern "C" fn(f32) -> f64 = unsafe {
            transmute(
                Self::ptr()[31]
            )
        };

        f(a)
    }
}