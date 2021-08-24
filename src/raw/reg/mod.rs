//! Register abstractions for the RP2040.


mod rreg;
mod xreg;

pub use self::rreg::RRegister;
pub use self::xreg::XRegister;


/// Trait common to all Registers.
pub trait Register<X> :
	core::ops::BitAnd<X> + core::ops::BitAndAssign<X> +
	core::ops::BitOr<X>  + core::ops::BitOrAssign<X>  +
	core::ops::BitXor<X> + core::ops::BitXorAssign<X>
{
	/// Reads the register from memory.
	fn read(&self) -> X;

	/// Write the value to the register.
	fn write(&mut self, val: X);

	/// Clears the given bits in the register.
	fn clear(&mut self, bits: X);

	/// Sets the given bits in the register.
	fn set(&mut self, bits: X);
}
