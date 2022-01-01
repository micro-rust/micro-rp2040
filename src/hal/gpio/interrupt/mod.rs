//! Abstraction to use a GPIO pin input as source for an IRQ.


pub(crate) mod core0;
pub(crate) mod core1;
//mod token;
mod trigger;


use crate::prelude::*;

use micro::asm::{ cpsid_i, cpsie_i };

//pub use self::token::GPIOInterruptToken;
pub use self::trigger::Trigger;



pub struct GPIOInterrupt<const N: u8>;

impl<const N: u8> GPIOInterrupt<N> {
    /// Offset from PROC0_INTE0 register.
    const R: usize = 4 * (N as usize / 8);

    /// Offset inside the INTEx register for the start of the control bits.
    const O: usize = 4 * (N as usize % 8);


    /// Creates a GPIOInterrupt from the given pin.
    /// This interrupt is deactivated when the returned object is dropped.
    pub fn reconfigurable(mut pin: Gpio<N>, handlers: [Option<(fn(), Trigger)>; 2], wake: Option<Trigger>) -> Self {
        // Clear previous configuration.
        pin.intclear();

        // Configure I/O state.
        Self::iostate();

        // Configure the interrupt.
        Self.config(handlers, wake);

        // Forget the GPIO to avoid callin its destructor.
        core::mem::forget(pin);

        Self
    }

    /// Creates a GPIOInterrupt from the given pin.
    /// This interrupt will be enabled forever and cannot be disabled nor modified.
    /// TODO : Create system to mask this interrupt.
    pub fn fixed(mut pin: Gpio<N>, handlers: [Option<(fn(), Trigger)>; 2], wake: Option<Trigger>) {
        // Clear previous configuration.
        pin.intclear();

        // Configure I/O state.
        Self::iostate();

        // Configure the interrupt.
        Self.config(handlers, wake);

        // Forget the GPIO to avoid callin its destructor.
        core::mem::forget(pin);
    }

    /// Configures the interrupt.
    pub fn config(&mut self, handlers: [Option<(fn(), Trigger)>; 2], wake: Option<Trigger>) {
        configure(N as usize, Self::R, Self::O, handlers, wake)
    }

    /// Configures the I/O state of the GPIO.
    fn iostate() {
        // Get CTRL register.
        let ctrl = unsafe { &mut *((<Gpio<N> as GPIOPin>::IO + 4) as *mut AtomicRegister<u32>) };
        ctrl.write( 0x2 << 12 );

        // Get PAD register.
        let pad = unsafe { &mut *(<Gpio<N> as GPIOPin>::PAD as *mut AtomicRegister<u32>) };
        pad.write((1 << 7) | (1 << 6) | (1 << 1) | 1);
    }
}


impl<const N: u8> Release for GPIOInterrupt<N> {
    fn release(&mut self) -> Result<(), SystemError> {
        crate::hal::gpio::common::intclear(Self::R, Self::O);

        loop {
            match Syslock::acquire() {
                Ok(_) => {
                    unsafe { ReleaseSystem::release( Gpio::<N>::id() )?; }
                    core::mem::forget(self);
                    return Ok(());
                },

                _ => (),
            }
        }
    }
}

impl<const N: u8> Drop for GPIOInterrupt<N> {
    fn drop(&mut self) {
        crate::hal::gpio::common::intclear(Self::R, Self::O);

        loop {
            match Syslock::acquire() {
                Ok(_) => {
                    unsafe { ReleaseSystem::force( Gpio::<N>::id() ); }
                    core::mem::forget(self);
                    break;
                },

                _ => (),
            }
        }
    }
}



/// Common function to clear the previous configuration.
#[inline(never)]
fn clear(r: usize, o: usize) {
    // Reference to the IRQ Raw registers.
    let intr = unsafe { &mut *(0x400140F0 as *mut [AtomicRegister<u32>; 40]) };

    // Clear triggerss
    intr[r +  0].clear(0xF << o);

    // Clear masks.
    intr[r +  4].clear(0xF << o);
    intr[r + 16].clear(0xF << o);
    intr[r + 28].clear(0xF << o);
}


/// Common function to configure an interrupt.
#[inline(never)]
fn configure(n: usize, r: usize, o: usize, handlers: [Option<(fn(), Trigger)>; 2], wake: Option<Trigger>) {
    // Disable interrupts.
    cpsid_i();

    // Reference to the IRQ enable registers.
    let inte = unsafe { &mut *(0x400140F0 as *mut [AtomicRegister<u32>; 40]) };

    // Configure the interrupt, wake and handlers.
    match handlers[0] {
        Some((handler, trigger)) => {
            // Set the Core 0 handler.
            unsafe { core0::HANDLERS[n] = handler as u32; }

            // Set the trigger.
            inte[r + 4].set((trigger as u32) << o);
        },

        _ => inte[r + 4].clear(0xF << o),
    }

    match handlers[1] {
        Some((handler, trigger)) => {
            // Set the Core 0 handler.
            unsafe { core1::HANDLERS[n] = handler as u32; }

            // Set the trigger.
            inte[r + 16].set((trigger as u32) << o);
        },

        _ => inte[r + 16].clear(0xF << o),
    }

    match wake {
        Some(trigger) => inte[r + 24].set((trigger as u32) << o),
        _ => inte[r + 28].clear(0xF << o),
    }

    // Clear sticky edge triggers.
    inte[r + 0].clear(0xF << o);

    // Restore interrupts.
    cpsie_i();
}
