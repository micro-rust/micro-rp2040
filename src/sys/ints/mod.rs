//! Interrupt module.

//pub(crate) mod handlers;

mod tables;



use crate::raw::{ SIORegister };

use micro::{ Register };

use self::tables::IRQTable;




pub struct InterruptSystem;

impl InterruptSystem {
    /// Enables the given IRQ.
    #[inline(always)]
    pub fn enableirq<const IRQ: u8>() {
        let ISER: &mut SIORegister<u32> = unsafe { &mut *(0xE000E100 as *mut _) };
        ISER.write( 1 << IRQ )
    }

    /// Disables the given IRQ.
    #[inline(always)]
    pub fn disableirq<const IRQ: u8>() {
        let ICER: &mut SIORegister<u32> = unsafe { &mut *(0xE000E180 as *mut _) };
        ICER.write( 1 << IRQ )
    }

    /// Enables the given IRQ.
    #[inline(always)]
    pub fn enableirqn(irq: u8) {
        let ISER: &mut SIORegister<u32> = unsafe { &mut *(0xE000E100 as *mut _) };
        ISER.write( 1 << irq )
    }

    /// Disables the given IRQ.
    #[inline(always)]
    pub fn disableirqn(irq: u8) {
        let ICER: &mut SIORegister<u32> = unsafe { &mut *(0xE000E180 as *mut _) };
        ICER.write( 1 << irq )
    }

    /// Returns `true` if the interrupt is enabled.
    #[inline(always)]
    pub fn enabled(irq: u8) -> bool {
        let ISER: &mut SIORegister<u32> = unsafe { &mut *(0xE000E100 as *mut _) };
        (ISER.read() & (1 << irq)) != 0
    }

    /// Sets the interrupt as pending.
    #[inline(always)]
    pub fn setpending(irq: u8) {
        let ISPR: &mut SIORegister<u32> = unsafe { &mut *(0xE000E200 as *mut _) };
        ISPR.write( 1 << irq )
    }

    /// Clers the interrupt as pending.
    #[inline(always)]
    pub fn clearpending(irq: u8) {
        let ICPR: &mut SIORegister<u32> = unsafe { &mut *(0xE000E280 as *mut _) };
        ICPR.write( 1 << irq )
    }

    /// Returns `true` if the interrupt is pending.
    #[inline(always)]
    pub fn pending(irq: u8) -> bool {
        let ISPR: &mut SIORegister<u32> = unsafe { &mut *(0xE000E200 as *mut _) };
        (ISPR.read() & (1 << irq)) != 0
    }

    /// Sets the PendSV exception flag.
    #[inline(always)]
    pub fn setPendSV() {
        let ICSR: &mut SIORegister<u32> = unsafe { &mut *(0xE000ED04 as *mut _) };
        ICSR.write( ICSR.read() | 1 << 28 )
    }

    /// Clears the PendSV exception flag.
    #[inline(always)]
    pub fn clearPendSV() {
        let ICSR: &mut SIORegister<u32> = unsafe { &mut *(0xE000ED04 as *mut _) };
        ICSR.write( ICSR.read() | 1 << 27 )
    }

    /// Sets the Systick exception flag.
    #[inline(always)]
    pub fn setSystick() {
        let ICSR: &mut SIORegister<u32> = unsafe { &mut *(0xE000ED04 as *mut _) };
        ICSR.write( ICSR.read() | 1 << 26 )
    }

    /// Clears the Systick exception flag.
    #[inline(always)]
    pub fn clearSystick() {
        let ICSR: &mut SIORegister<u32> = unsafe { &mut *(0xE000ED04 as *mut _) };
        ICSR.write( ICSR.read() | 1 << 25 )
    }

    /// Configures the IRQ with the given function and priority level.
    /// The priority goes from 0 to 3, with 0 being highest priority.
    pub(crate) fn configure<const IRQ: usize>(f: fn(), prio: u8) {
        use micro::asm::*;

        let R: usize = IRQ / 4;
        let O: usize = ((IRQ % 4) * 8) + 6;

        let IPR: &mut [SIORegister<u32>; 8] = unsafe { &mut *(0xE000E400 as *mut _) };

        // Set the priority.
        IPR[R].write( (IPR[R].read() & !(0x3 << O)) | ((prio as u32) << O) );

        // Enable the IRQ.
        Self::enableirqn(IRQ as u8);

        // Set the function.
        match crate::sys::coreid() {
            0 => IRQTable::at(0x20040040).set::<IRQ>(f),
            _ => IRQTable::at(0x20040840).set::<IRQ>(f),
        }

        // Set all memory barriers.
        isb();
        dmb();
        dsb();
    }
}
