//! Arduino style access library to interact with RP2040 microcontrollers.

#![no_std]
#![feature(asm)]
#![feature(generic_const_exprs)]
#![feature(const_fn_trait_bound)]



/// Power and Reset peripherals.
pub mod power;

/// Synchronization module.
pub mod sync;

/// System module.
pub mod sys;



/// Raw access module.
pub(crate) mod raw;