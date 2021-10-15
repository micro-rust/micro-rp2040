//! Feature-gated configuration data.



#[cfg(feature = "xosc-8MHz" )]
#[no_mangle]
#[used]
pub static __XFREQ__ : u32 =  8_000_000;

#[cfg(feature = "xosc-10MHz")]
#[no_mangle]
#[used]
pub static __XFREQ__ : u32 = 10_000_000;

#[cfg(feature = "xosc-12MHz")]
#[no_mangle]
#[used]
pub static __XFREQ__ : u32 = 12_000_000;

#[cfg(feature = "xosc-15MHz")]
#[no_mangle]
#[used]
pub static __XFREQ__ : u32 = 15_000_000;



#[cfg(feature = "adafruit")]
static __XOSCMUL__ : u32 = 64;

#[cfg(not(feature = "adafruit"))]
static __XOSCMUL__ : u32 = 1;




#[no_mangle]
#[used]
pub static __DELAY__ : u32 = (((__XFREQ__ / 1000) + 128) / 256) * __XOSCMUL__;




#[cfg(feature = "flash-2MB" )]
#[no_mangle]
#[used]
pub static FLASHSIZE : u32 =  2 * 1024 * 1024;

#[cfg(feature = "flash-4MB" )]
#[no_mangle]
#[used]
pub static FLASHSIZE : u32 =  4 * 1024 * 1024;

#[cfg(feature = "flash-8MB" )]
#[no_mangle]
#[used]
pub static FLASHSIZE : u32 =  8 * 1024 * 1024;

#[cfg(feature = "flash-16MB")]
#[no_mangle]
#[used]
pub static FLASHSIZE : u32 = 16 * 1024 * 1024;

