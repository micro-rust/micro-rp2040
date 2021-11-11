//! Core 0 Reset function.
//! Initializes and configures the basic peripherals for the system.
//! Configures and allows the use of the pseudo-kernel's functions.


mod initialize;
mod jump;
mod sections;
mod wakeup;


#[link_section = ".vectortable.Reset0"]
#[no_mangle]
#[used]
static RESET0 : fn() -> ! = Reset0;



pub(crate) fn Reset0() -> ! {
    unsafe { sections::sections(); }

    initialize::initialize();

    wakeup::wakeup();

    jump::jump0()
}
