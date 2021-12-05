//! `micro-rp2040` blinky example.

#![no_std]
#![no_main]

#![allow(unused_imports)]


#![no_std]
#![no_main]

#![allow(unused_imports)]


// Give the crate a prettier name, can skip.
use micro_rp2040 as rp2040;


// Import the prelude of the crate imports the basic traits needed to interact with the framework.
use rp2040::prelude::*;


// Import Gpio to be able to use the pin.
// Import LedPin, an abstraction over LED connected to pins.
use rp2040::hal::pins::{ Gpio, led::LedPin };


// Use this to import a safe abstraction over the nop and bkpt assembly instructions.
use micro::asm::*;


// Give the linker the pointer of the main function.
// This step is mandatory, as the library does not have direct access to your own code.
main0!(usermain);


fn usermain() -> ! {
    // Acquire PIN 25. This is the LED pin in Raspberry-Pico.
    // Change to your boards' (non-addressable / non-Neopixel) LED pin.
    let led = Gpio::<25>::acquire().unwrap();

    // Initialize the GPIO into LED mode.
    led.init();

    loop {
        // Turn the LED on.
        led.on();

        // This loop takes three cycles, if running at 133 MHz, this is a 1 second loop.
        for _ in 0..44_444_444 { nop() }

        // Turn the LED off.
        led.off();

        // This loop takes three cycles, if running at 133 MHz, this is a 1 second loop.
        for _ in 0..44_444_444 { nop() }
    }
}

// The user must define their own panic handler.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { bkpt::<255>() }
}
