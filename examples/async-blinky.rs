//! `micro-rp2040` asynchronous blinky.
//! The difference with the normal blinky, the Systick exception will control
//! the LED and the time interval, leaving the CPU free to do other things.

#![no_std]
#![no_main]

#![allow(unused_imports)]


// Give the crate a prettier name, can skip.
use micro_rp2040 as rp2040;


// Import the macro to declare the main function for core 0.
use rp2040::{ main0, systick };


// Import Gpio to be able to use the pins.
use rp2040::pins::Gpio;
// LedPin is a safe optimized abstraction over a pin that drives an led.
use rp2040::pins::led::LedPin;


// Import the Systick abstraction.
use rp2040::time::systick::Systick;


// Use this to import a safe abstraction over the nop and bkpt assembly instructions.
use micro::asm::*;


// Static to hold the LED Pin to pass from the main function once initialized.
#[used]
static mut LED : Option<LedPin<25>> = None;



// Give the linker the pointer of the main function.
main0!(usermain);


fn usermain() -> ! {
    // Acquire the local Systick.
    let mut systick = Systick::acquire().unwrap();

    // Set a reload value for 1 second.
    systick.reload(66_666_666);

    // Set Systick source clock as the system clock.
    systick.processor();

    // Enable the Systick interrupt.
    systick.interrupt(true);

    // Enable the Systick.
    systick.enable();

    loop {
        // You can do stuff here.
        // The LED will blink on its own.
        // I, for example, am calculating what's 2 + 2, cause my calculator broke.
        let x = 2;
        let y = 2;

        let z = x + y;
    }
}



// Define the Systick interrupt for the Core 0.
#[no_mangle]
unsafe extern "C" fn Systick0() {
    static mut STATE : bool = false;
    static mut LED : Option<LedPin<25>> = None;

    match unsafe { &LED } {
        // If the Pin has been acquired, toggle it.
        Some(ref led) => unsafe {
            if STATE { led.off(); STATE = false; }
            else { led.on(); STATE = true; }
        },

        // If the Pin has not been acquired yet, acquire it and turn it on.
        None => {
            // Acquire PIN 25.
            let led: LedPin<25> = LedPin::from(Gpio::<25>::acquire().unwrap());

            // Initialize the LED.
            led.init();

            unsafe { LED = Some(led); }
        },
    }
}


// The user must define their own panic handler.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop { bkpt::<255>() }
}
