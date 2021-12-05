//! Initialization procedure of the System Data regions.

extern "C" {
    static mut __ssysdata1 : u32;
    static mut __esysdata1 : u32;
    static     __lsysdata1 : u32;
}

pub(crate) fn sysdata() {
    unsafe { crate::sys::init::load(
        &mut __ssysdata1 as *mut u32,
        &mut __esysdata1 as *mut u32,
        & __lsysdata1 as *const u32,
    )};
}
