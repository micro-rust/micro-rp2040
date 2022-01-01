//! Core 1 Reset function.
//! Initializes and configures the basic peripherals for the system.
//! Configures and allows the use of the pseudo-kernel's functions.



mod init;

use init::{ mpu, romfunc, sysdata, vectortable };

#[link_section = ".vectortable.Reset1"]
#[no_mangle]
#[used]
static RESET1 : fn() -> ! = Reset1;



pub(crate) fn Reset1() -> ! {
    // Initialize the Vector Table.
    vectortable();

    // Load the System Data 0.
    sysdata();

    // Set up MPU with guard stack.
    mpu();

    // Wait for all DMA channels to end.
    crate::sys::init::dmawait();

    micro::asm::dmb();
    micro::asm::dsb();
    micro::asm::isb();

    // Load ROM functions.
    unsafe { romfunc(); }

    // Wait for confirmation that Core 0 has finished.
    // Send a message to Core 0 indicating initialization has ended.
    crate::sys::init::corewait();

    // Jump to user code.
    extern "C" {
        static __MAINFN1 : extern fn() -> !;
    }

    match unsafe { __MAINFN1 } as u32 {
        0x00000000 => loop { micro::asm::nop(); },
        _ => unsafe { __MAINFN1() },
    }
}
