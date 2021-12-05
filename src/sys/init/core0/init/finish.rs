//! Setup finish procedure.

use crate::prelude::*;
use crate::features::__XFREQ__;
use crate::sys::{ CLOCKS, clocks::Clock };

pub(crate) fn finish() {
    // Wait for all DMA channels to finish.
    for i in 0..4 {
        let dma = unsafe { &mut *((0x50000000 + (i * 0x40)) as *mut [AtomicRegister<u32>; 4]) };

        'inner: loop {
            if (dma[3].read() & (1 << 24)) == 0 {
                break 'inner;
            }
        }
    }


    // Load the default clock values.
    unsafe {
        // Default frequency of System PLL, System and Peripheral is 125 MHz.
        CLOCKS[Clock::PllSys.index()] = 125_000_000;
        CLOCKS[Clock::System.index()] = 125_000_000;
        CLOCKS[Clock::Peripheral.index()] = 125_000_000;

        // Default frequency of Reference and Crystal Oscillator is XFREQ.
        CLOCKS[Clock::Reference.index()] = __XFREQ__;
        CLOCKS[Clock::Xosc.index()] = __XFREQ__;

        // Default frequency of USB PLL, USB and ADC is 48 MHz.
        CLOCKS[Clock::PllUsb.index()] = 48_000_000;
        CLOCKS[Clock::Usb.index()] = 48_000_000;
        CLOCKS[Clock::Adc.index()] = 48_000_000;

        // Default frequency of RTC is 48 kHz.
        CLOCKS[Clock::Rtc.index()] = 48_000;
    }
}