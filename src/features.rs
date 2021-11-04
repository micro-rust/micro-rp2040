//! Feature-gated configuration data.



#[cfg(feature = "xosc-8MHz" )]
pub const __XFREQ__ : u32 =  8_000_000;

#[cfg(feature = "xosc-10MHz")]
pub const __XFREQ__ : u32 = 10_000_000;

#[cfg(feature = "xosc-12MHz")]
pub const __XFREQ__ : u32 = 12_000_000;

#[cfg(feature = "xosc-15MHz")]
pub const __XFREQ__ : u32 = 15_000_000;



#[cfg(feature = "adafruit")]
const __XOSCMUL__ : u32 = 64;

#[cfg(not(feature = "adafruit"))]
const __XOSCMUL__ : u32 = 1;



pub const __DELAY__ : u32 = (((__XFREQ__ / 1000) + 128) / 256) * __XOSCMUL__;




#[cfg(feature = "flash-2MB" )]
pub const FLASHSIZE : u32 =  2 * 1024 * 1024;

#[cfg(feature = "flash-4MB" )]
pub const FLASHSIZE : u32 =  4 * 1024 * 1024;

#[cfg(feature = "flash-8MB" )]
pub const FLASHSIZE : u32 =  8 * 1024 * 1024;

#[cfg(feature = "flash-16MB")]
pub const FLASHSIZE : u32 = 16 * 1024 * 1024;

