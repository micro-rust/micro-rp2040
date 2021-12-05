//! Initialization procedure of the Subsystem resets.

use crate::prelude::*;

pub(crate) fn reset() {
    // Reference to the Subsystem Reset register.
    let reset = unsafe { &mut *(0x4000C000 as *mut [AtomicRegister<u32>; 3]) };

    // Reset everything except QSPI IO/PADS, SYS and USB PLLs, SYSCFG and USBCTRL.
    reset[0].set(!0x01043240);
    delay();

    // Unreset everything except ADC, RTC, SPI and UART.
    reset[0].clear(!0x00C38002);
    delay();
}


// Delay function to propagate resets.
#[inline(always)]
fn delay() {
	for _ in 0..20 {
		micro::asm::nop();
	}
}