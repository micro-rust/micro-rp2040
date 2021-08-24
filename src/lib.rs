//! Arduino style access library to interact with RP2040 microcontrollers.

#![no_std]
#![feature(asm)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

/// Low level access and misc. items.
pub mod raw;

/// System resources and configuration module.
pub mod sys;

