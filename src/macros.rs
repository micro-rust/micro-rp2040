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


/// Macro to reserve GPIOs at compile time.
/// This will fail with a linker error if a pin is reserved more than once.
/// Example:
/// ```
/// static PIN0 : Gpio<0> = reserveGPIO!(0);
/// ```
#[macro_export]
macro_rules! reserveGPIO {
	($x:expr) => {{
		#[no_mangle]
		static __GPIOX__ : u32 = (1 << $x);

		unsafe { micro_rp2040::pins::Gpio::<$x>::reserve() }
	}};
}
