//! Jump to Core 1 user main.
//! If there is no user code, hangs in a debug loop.


pub(super) fn jump1() -> ! {
    extern "C" {
        static __MAINFN1 : extern fn() -> !;
    }

    match unsafe { __MAINFN1 } as u32 {
        0x00000000 => loop { micro::asm::bkpt::<136>(); },
        _ => unsafe { __MAINFN1() },
    }
}
