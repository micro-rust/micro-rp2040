//! Feature-gated configuration data.



#[cfg(feature = "xosc-8MHz" )]
#[no_mangle]
#[used]
static XFREQ : u32 =  8_000_000;

#[cfg(feature = "xosc-10MHz")]
#[no_mangle]
#[used]
static XFREQ : u32 = 10_000_000;

#[cfg(feature = "xosc-12MHz")]
#[no_mangle]
#[used]
static XFREQ : u32 = 12_000_000;

#[cfg(feature = "xosc-15MHz")]
#[no_mangle]
#[used]
static XFREQ : u32 = 15_000_000;




#[cfg(feature = "flash-2MB" )]
#[no_mangle]
#[used]
static FLASHSIZE : u32 =  2 * 1024 * 1024;

#[cfg(feature = "flash-4MB" )]
#[no_mangle]
#[used]
static FLASHSIZE : u32 =  4 * 1024 * 1024;

#[cfg(feature = "flash-8MB" )]
#[no_mangle]
#[used]
static FLASHSIZE : u32 =  8 * 1024 * 1024;

#[cfg(feature = "flash-16MB")]
#[no_mangle]
#[used]
static FLASHSIZE : u32 = 16 * 1024 * 1024;

