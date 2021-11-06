//! Core 1 Reset function.
//! Initializes and configures the basic peripherals for the system.
//! Configures and allows the use of the pseudo-kernel's functions.


#[link_section = ".vectortable.Reset1"]
#[no_mangle]
#[used]
static RESET1 : fn() -> ! = Reset1;



pub(crate) fn Reset1() -> ! {
    // Initialize the interrupts.
    initialize();

    // Jump to user code.
    jump1()
}


fn initialize() {
    // Initialize interrupts.
    crate::sys::ints::InterruptSystem::init();
}

/// Jumps to user code.
/// If there is no user code, hangs in a debug loop.
fn jump1() -> ! {
    extern "C" {
        static __MAINFN1 : extern fn() -> !;
    }

    match unsafe { __MAINFN1 } as u32 {
        0x00000000 => loop { micro::asm::bkpt::<136>(); },
        _ => unsafe { __MAINFN1() },
    }
}
