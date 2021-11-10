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


//mod alloc;



/// Error module.
pub mod error;

/// Peripehrals module.
pub mod peripherals;

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
