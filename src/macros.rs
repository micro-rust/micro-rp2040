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
#[macro_export]
macro_rules! defineXOSC {
	($f:expr) => {
		#[no_mangle]
		#[used]
		static __XOSC : u32 = $f;
	};
}