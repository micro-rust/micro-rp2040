//! Jumps to the user's main function for core 0.

/// Jumps to user code.
/// If there is no user code, hangs in a debug loop.
pub(super) fn jump0() -> ! {
    extern "C" {
        static __MAINFN0 : extern fn() -> !;
    }

    match unsafe { __MAINFN0 } as u32 {
        0x00000000 => loop { micro::asm::bkpt::<255>(); },
        _ => unsafe { __MAINFN0() },
    }
}
