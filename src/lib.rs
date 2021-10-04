//! Arduino style access library to interact with RP2040 microcontrollers.

#![no_std]
#![feature(asm)]
#![feature(generic_const_exprs)]
#![feature(const_fn_trait_bound)]


#![allow(incomplete_features)]


/// GP I/O Pins module.
pub mod pins;

/// Power and Reset peripherals.
pub mod power;

/// Synchronization module.
pub mod sync;

/// System module.
pub mod sys;

/// Time module.
pub mod time;


/// Raw access module.
pub(crate) mod raw;



/// Macro module.
#[macro_use]
mod macros;

/// Feature gated details.
mod features;