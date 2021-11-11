//! Initialization of the .systembss and .systemdata sections.

use crate::sys::init::{ load, zero };

pub(super) unsafe fn sections() {
    extern "C" {
        static mut __ssysdata : u32;
        static mut __esysdata : u32;
        static __lsysdata : u32;
    }

    load(
        &mut __ssysdata as *mut u32,
        &mut __esysdata as *mut u32,
        & __lsysdata as *const u32,
    );

    extern "C" {
        static mut __ssysbss : u32;
        static mut __esysbss : u32;
    }

    zero(
        &mut __ssysbss as *mut u32,
        &mut __esysbss as *mut u32,
    );
}
