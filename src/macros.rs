//! Macro module.



/// Builds an exported pointer to the user's main function for Core 0.
#[macro_export]
macro_rules! main0 {
    ($func:ident) => {
        #[no_mangle]
        #[used]
        static __MAINFN0 : fn() -> ! = $func;
    };
}

/// Builds an exported pointer to the user's main function for Core 1.
#[macro_export]
macro_rules! main1 {
    ($func:ident) => {
        #[no_mangle]
        #[used]
        static __MAINFN1 : fn() -> ! = $func;
    };
}

/// Macro exported to be used when the XOSC frequency is unknown.
#[cfg(not(feature = "xosc-defined"))]
#[macro_export]
macro_rules! defineXOSC {
    ($f:expr) => {
        #[no_mangle]
        #[used]
        static XFREQ : u32 = $f;
    };
}

/// Macro exported to be used when the Flash size is unknown.
#[cfg(not(feature = "flash-defined"))]
#[macro_export]
macro_rules! defineFLASHSIZE {
    ($f:expr) => {
        #[no_mangle]
        #[used]
        static FLASHSIZE : u32 = $f;
    };
}
