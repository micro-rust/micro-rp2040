//! Core 0 IRQ handlers for GPIO interrupts.


#[link_section = ".sysbss1.GPIOIRQ"]
pub(super) static mut HANDLERS: [u32; 30] = [0u32; 30];
