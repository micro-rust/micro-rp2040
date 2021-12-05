//! Initialization procedure of the System Data regions.

extern "C" {
    static mut __ssysdata0 : u32;
    static mut __esysdata0 : u32;
    static     __lsysdata0 : u32;
}


pub(crate) fn sysdata() {
    unsafe { crate::sys::init::load(
        &mut __ssysdata0 as *mut u32,
        &mut __esysdata0 as *mut u32,
        & __lsysdata0 as *const u32,
    )}
}

extern "C" {
    static mut __sdata : u32;
    static mut __edata : u32;
    static     __ldata : u32;
}

pub(crate) fn data() {
    unsafe { crate::sys::init::load(
        &mut __sdata as *mut u32,
        &mut __edata as *mut u32,
        & __ldata as *const u32,
    )};
}
