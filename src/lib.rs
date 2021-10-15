//! Arduino style access library to interact with RP2040 microcontrollers.

#![no_std]
#![feature(asm)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(const_raw_ptr_deref)]
#![feature(const_fn_trait_bound)]
#![feature(inherent_associated_types)]


#![allow(dead_code)]
#![allow(incomplete_features)]
#![allow(non_snake_case)]


/// Communication peripherals.
pub mod comm;

/// Interrupt module.
#[macro_use]
pub mod ints;

/// GP I/O Pins module.
pub mod pins;

/// Power and Reset peripherals.
pub mod power;

/// Prelude module.
pub mod prelude;

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