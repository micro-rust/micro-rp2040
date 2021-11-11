//! Core 1 Reset function.
//! Initializes and configures the basic peripherals for the system.
//! Configures and allows the use of the pseudo-kernel's functions.


mod initialize;
mod jump;

#[link_section = ".vectortable.Reset1"]
#[no_mangle]
#[used]
static RESET1 : fn() -> ! = Reset1;



pub(crate) fn Reset1() -> ! {
    initialize::initialize();

    // Jump to user code.
    jump::jump1()
}
