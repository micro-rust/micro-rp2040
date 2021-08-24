//! System module. Access system resources and configuration.
//! Emulates a pseudo kernel that gives some safety guarantees and access to
//! low level hardware and MCU configuration.
//! 
//! # Modules
//! 
//! ## `local`
//! The `local` module gives access to hardware peripherals with core exclusive
//! access, making them safe to use.
//! 
//! ## `shared`
//! The `shared` module gives access to hardware peripherals that are NOT core
//! exclusive, and their use might create data races or corruption. The crate
//! guarantees that these races do not occur with the use of mutexes.

pub mod local;