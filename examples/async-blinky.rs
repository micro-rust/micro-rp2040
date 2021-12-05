//! `micro-rp2040` asynchronous blinky.
//! The difference with the normal blinky, the Systick exception will control
//! the LED and the time interval, leaving the CPU free to do other things.

#![no_std]
#![no_main]

#![allow(unused_imports)]


// Give the crate a prettier name, can skip.
use micro_rp2040 as rp2040;


// Import the prelude of the crate imports the basic traits needed to interact with the framework.
use rp2040::prelude::*;


// Import the Systick abstraction.
use rp2040::time::Systick;


// Use this to import a safe abstraction over the nop and bkpt assembly instructions.
use micro::asm::*;


// Give the linker the pointer of the main function.
// This step is mandatory, as the library does not have direct access to your own code.
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

        let _z = x + y;
    }
}



// Define the Systick interrupt for the Core 0.
#[no_mangle]
unsafe extern "C" fn Systick0() {
    // Import Gpio to be able to use the pin.
    // Import LedPin, an abstraction over LED connected to pins.
    use rp2040::hal::pins::{ Gpio, led::LedPin };


    static mut STATE : bool = false;
    static mut LED : Option<Gpio<25>> = None;

    match &LED {
        // If the Pin has been acquired, toggle it.
        Some(ref led) => {
            if STATE { led.off(); STATE = false; }
            else { led.on(); STATE = true; }
        },

        // If the Pin has not been acquired yet, acquire it and turn it on.
        None => {
            // Acquire PIN 25.
            let led = Gpio::<25>::acquire().unwrap();

            // Initialize the LED.
            led.init();

            LED = Some(led);
        },
    }
}


// The user must define their own panic handler.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { bkpt::<255>() }
}
