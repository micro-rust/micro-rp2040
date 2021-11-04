//! Core 0 Reset function.
//! Initializes and configures the basic peripherals for the system.
//! Configures and allows the use of the pseudo-kernel's functions.


#[link_section = ".vectortable.Reset0"]
#[no_mangle]
#[used]
static RESET0 : fn() -> ! = Reset0;



fn Reset0() -> ! {
    // Load RAM sections.
    unsafe { sections(); }

    // Initialize the clocks.
    initialize();

    micro::asm::bkpt::<2>();

    // Jump to user code.
    jump()
}



/// Loads the RAM sections and zeros the .bss and .uninit.
unsafe fn sections() {
    use super::load;
    use super::zero;

    extern "C" {
        static mut __sdata : u32;
        static mut __edata : u32;
        static __ldata : u32;
    }

    load(
        &mut __sdata as *mut u32,
        &mut __edata as *mut u32,
        & __ldata as *const u32,
    );

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
        static mut __sbss : u32;
        static mut __ebss : u32;
    }

    zero(
        &mut __sbss as *mut u32,
        &mut __ebss as *mut u32,
    );

    extern "C" {
        static mut __suninit : u32;
        static mut __euninit : u32;
    }

    zero(
        &mut __suninit as *mut u32,
        &mut __euninit as *mut u32,
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


/// Initialize the RP2040.
fn initialize() {
    // Initialization routine of the Reset peripheral.
    crate::power::RESET.init();

    // Initialization routine of the Clocks peripheral.
    unsafe { crate::time::CLOCKS.init(); }

    // Initialize interrupts.
    crate::ints::InterruptSystem::init();
}


/// Jumps to user code.
/// If there is no user code, hangs in a debug loop.
fn jump() -> ! {
    extern "C" {
        static __MAINFN0 : extern fn() -> !;
    }

    match unsafe { __MAINFN0 } as u32 {
        0x00000000 => loop { micro::asm::bkpt::<255>(); },
        _ => unsafe { __MAINFN0() },
    }
}