//! Arduino style access library to interact with RP2040 microcontrollers.

#![no_std]
#![feature(asm)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(const_fn_trait_bound)]
#![feature(inherent_associated_types)]
//#![feature(const_maybe_uninit_assume_init)]

#![allow(dead_code)]
#![allow(drop_bounds)]
#![allow(incomplete_features)]
#![allow(non_snake_case)]


//mod alloc;



/// Alloc module.
#[cfg(feature = "alloc")]
mod alloc;

/// Error module.
pub mod error;

/// HW and SW math abstractions.
pub mod math;

/// Peripehrals module.
pub mod peripherals;

pub mod hal;

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
