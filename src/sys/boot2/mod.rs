//! Module to include the stage 2 bootloader into the final binary.



#[cfg(not(feature = "flash-defined"))]
mod generich03h;


#[cfg(feature = "w25q080")]
mod w25q080;