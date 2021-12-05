//! Initialization procedure of the system Clocks.


use crate::features::{ __DELAY__, __XFREQ__ };
use crate::prelude::*;


pub(crate) fn clocks() {
    // PREINIT sequence.
    // ************************************************************************

    // Precalculated Watchdog configuration.
    #[link_section = ".rodata"]
    static WATCHDOG: u32 = (1 << 9) | (__XFREQ__ / 1_000_000);

    // Enable Watchdog tick.
    let watchdog = unsafe { &mut *(0x40058000 as *mut [AtomicRegister<u32>; 12]) };
    watchdog[11].write(WATCHDOG);

    // Disable Resuscitation clock.
    let resus = unsafe { &mut *(0x40008078 as *mut [AtomicRegister<u32>; 2]) };
    resus[0].write(0);

    // Switch reference and system clocks to their defaults.
    let reference = unsafe { &mut *(0x40008030 as *mut [AtomicRegister<u32>; 2]) };
    reference[0].write(0);

    let system = unsafe { &mut *(0x4000803C as *mut [AtomicRegister<u32>; 2]) };
    system[0].write(0);

    // ************************************************************************



    // INIT CLOCK SOURCES sequence.
    // ************************************************************************

    // Reference to the System and USB PLLs.
    let pllsys = unsafe { &mut *(0x40028000 as *mut [AtomicRegister<u32>; 4]) };
    let pllusb = unsafe { &mut *(0x4002C000 as *mut [AtomicRegister<u32>; 4]) };


    // Turn off the System and USB PLLs.
    pllsys[1].write(0xFF);
    pllusb[1].write(0xFF);

    // Initialize the Crystal Oscillator.
    xosc();

    // Initialize the System and USB PLL.
    sys();
    usb();

    // ************************************************************************



    // INIT CLOCK OUTPUTS sequence.
    // ************************************************************************

    // Configure System clock.
    sysout();

    // Configure Reference clock.
    refout();

    // Configure Peripheral clock.
    auxclock(0x40008048, 1 << 8, 0);

    // Configure USB clock.
    auxclock(0x40008054, 1 << 8, 0);

    // Configure ADC clock.
    auxclock(0x40008060, 1 << 8, 0);

    // Configure RTC clock.
    auxclock(0x4000806C, 1000 << 8, 0);

    // ************************************************************************
}


/// Configures the System clock.
#[inline(never)]
fn sysout() {
    // Reference to the Clock Output registers.
    let clock = unsafe { &mut *(0x4000803C as *mut [AtomicRegister<u32>; 3]) };

    // Swicth to the reference clock.
    clock[0].write(0);

    // Write the divider.
    clock[1].write(1 << 8);

    // Swicth to the AUX source 0.
    clock[0].set(1);

    // Wait until the clock is stable.
    while clock[2].read() == 0 { micro::asm::nop() }
}

/// Configures the System clock.
#[inline(never)]
fn refout() {
    // Reference to the Clock Output registers.
    let clock = unsafe { &mut *(0x40008030 as *mut [AtomicRegister<u32>; 3]) };

    // Swicth to the reference clock.
    clock[0].write(0);

    // Write the divider.
    clock[1].write(1 << 8);

    // Swicth to the AUX source 0.
    clock[0].write(0x2);

    // Wait until the clock is stable.
    while clock[2].read() == 0 { micro::asm::nop() }
}

/// Enables the default non-glitchless clocks.
#[inline(never)]
fn auxclock(addr: u32, div: u32, src: u32) {
    // Reference to the Clock Output registers.
    let clock = unsafe { &mut *(addr as *mut [AtomicRegister<u32>; 3]) };

    // Stop the clock cleanly.
    clock[0].write(0);

    // Set the divisor.
    clock[1].write(div);

    // Configure the source and enable.
    clock[0].write((1 << 11) | (src << 5));
}



/// Initializes the Crystal Oscillator.
fn xosc() {
    // Reference to the Crystal Oscillator.
    let xosc = unsafe { &mut *(0x40024000 as *mut [AtomicRegister<u32>; 4]) };

    // Set input frequency.
    xosc[0].write(0xAA0);

    // Set startup delay.
    xosc[3].write(__DELAY__);

    // Set enable status and frequency range.
    xosc[0].write( (0xFAB << 12) | 0xAA0 );

    // Wait for stable XOSC.
    while (xosc[1].read() >> 31) == 0 { micro::asm::nop() }

    // Clear possible initial bad write.
    // This is a hardware bug that happens sometimes even on a good initialization.
    xosc[1].set(1 << 24);
}



/// Initializes the given PLL with the given configuration.
#[inline(never)]
fn pll(addr: u32, cfg: u32) {
    // Create the PLL reference.
    let pll = unsafe { &mut *(addr as *mut [AtomicRegister<u32>; 4]) };

    // Load reference divisor and feedback divisor.
    pll[0].write(1);
    pll[2].write(cfg);

    // Turn on PLL and VCO domains and wait for stabilization.
    pll[1].clear((1 << 5) | 1);
    while pll[0].read() >> 31 == 0 { micro::asm::nop() }

    // Load post dividers.
    pll[3].write(cfg);

    // Turn on post dividers.
    pll[1].clear(1 << 3);
}

/// Initializes the System PLL to 125 MHz low jitter.
#[inline(always)]
fn sys() {
    // Precalculated System PLL configuration.
    #[link_section = ".rodata"]
    static __CONFIG__: u32 = (6 << 16) | (2 << 12) | ( 1500000 / (__XFREQ__ / 1000));

    pll(0x40028000, __CONFIG__);
}

/// Initializes the USB PLL to 48 MHz low jitter.
fn usb() {
    // Precalculated USB PLL configuration.
    #[link_section = ".rodata"]
    static __CONFIG__: u32 = (6 << 16) | (5 << 12) | ( 1440000 / (__XFREQ__ / 1000));

    pll(0x4002C000, __CONFIG__);
}
