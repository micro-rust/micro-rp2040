//! `micro-rp2040` blinky example.

#![no_std]
#![no_main]

#![allow(unused_imports)]


// Give the crate a prettier name, can skip.
use micro_rp2040 as rp2040;

// Import the macro to declare the main function for core 0.
use rp2040::{ main0 };

// Import Gpio to be able to use the pins.
use rp2040::pins::Gpio;
// LedPin is a safe optimized abstraction over a pin that drives an led.
use rp2040::pins::led::LedPin;

// Use this to import a safe abstraction over the nop and bkpt assembly instructions.
use micro::asm::*;


// Give the linker the pointer of the main function.
main0!(usermain);


fn usermain() -> ! {
    // Acquire PIN 25. This is the LED pin in Raspberry-Pico.
    // Change to your boards' (non-addressable / non-Neopixel) LED pin.
    let led: LedPin<25> = LedPin::from(Gpio::<25>::acquire().unwrap());

    // Initialize the LED.
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
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop { bkpt::<255>() }
}
