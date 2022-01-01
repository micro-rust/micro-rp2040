//! Core 0 Reset function.
//! Initializes and configures the basic peripherals for the system.
//! Configures and allows the use of the pseudo-kernel's functions.


mod init;
mod wakeup;



use init::{ bss, clocks, data, finish, mpu, reset, sysdata, vectortable };
use wakeup::wakeup;


#[link_section = ".vectortable.Reset0"]
#[no_mangle]
#[used]
static RESET0 : fn() -> ! = Reset0;




pub(crate) fn Reset0() -> ! {
    // Reset the unnecesary subsystems.
    reset();

    // Initialize the BSS sections using DMA to speed up.
    bss();


    // Wakeup Core 1 to speed up init.
    wakeup();


    // Initialize the clocks.
    // Safe to do because the system clock is glitchless.
    clocks();


    // Initialize the Vector Table.
    vectortable();


    // Load the System Data 0.
    sysdata();


    // Load the program Data.
    data();


    // Set up MPU with guard stack.
    mpu();


    // Finish the setup.
    finish();


    // Wait for confirmation that Core 1 has finished.
    // Send a message to Core 1 indicating initialization has ended.
    crate::sys::init::corewait();

    // Jump to user code or hang if no user code is found.
    extern "C" {
        static __MAINFN0 : extern fn() -> !;
    }

    match unsafe { __MAINFN0 } as u32 {
        0x00000000 => loop { micro::asm::nop(); },
        _ => unsafe { __MAINFN0() },
    }
}


