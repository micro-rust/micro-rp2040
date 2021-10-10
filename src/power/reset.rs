//! Reset subsystem.
//! Allows Resetting and pulling out of reset different peripherals.


use crate::raw::Peripheral;
use crate::raw::AtomicRegister;


use micro::Register;
use micro::asm::nop;


static mut RESET : Peripheral<u32, AtomicRegister<u32>, 3, 0x4000C000> = Peripheral::get();


pub struct ResetSystem;

impl ResetSystem {
	/// Static initializer.
	pub const fn empty() -> Self {
		ResetSystem
	}

	/// Performs the intialization routine.
	pub(crate) fn init(&self) {
		static RES : ResetId =
				ResetId::IOQSPI + ResetId::PADSQSPI +
				ResetId::PLLSYS + ResetId::PLLUSB +
				ResetId::USBCTRL + ResetId::SYSCFG;

		static UNRES : ResetId =
				ResetId::ADC + ResetId::RTC +
				ResetId::SPI0 + ResetId::SPI1 +
				ResetId::UART0 + ResetId::UART1;

		unsafe {
			// Reset all but IOQSPI, PADSQSPI, PLLSYS, PLLUSB, USBCTRL, SYSCFG.
			RESET[0].set(u32::from(RES.inverse()));

			// Unreset all but ADC, RTC, SPIx and UARTx.
			RESET[0].clear(u32::from(UNRES.inverse()));

			// Wait until all periopherals are accessible.
			while (RESET[2].read() & u32::from(UNRES)) == 0 { micro::asm::nop(); }
		}
	}

	/// Puts the given peripherals in Reset.
	pub fn reset(&self, id: ResetId) {
		unsafe { RESET[0].set(u32::from(id)) };
	}

	/// Pulls the given peripherals from Reset.
	pub fn unreset(&self, id: ResetId) {
		unsafe { RESET[0].clear(u32::from(id)) };
	}

	/// Crate internal rset cycle.
	pub(crate) fn cycle(&self, id: ResetId) {
		// Reset.
		unsafe { RESET[0].set(u32::from(id)) };

		// Wait for propagation.
		for _ in 0..20 { nop() }

		// Unreset.
		unsafe { RESET[0].clear(u32::from(id)) };

		// wait until unreset is done.
		while ( unsafe { RESET[2].read() } & u32::from(id)) == 0 { nop(); }
	}
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ResetId(u32);

impl ResetId {
	pub const ADC : ResetId = ResetId(1 << 0);

	pub const BUSCTRL : ResetId = ResetId(1 << 1);

	pub const DMA : ResetId = ResetId(1 << 2);

	pub const I2C0 : ResetId = ResetId(1 << 3);
	pub const I2C1 : ResetId = ResetId(1 << 4);

	pub const IOBANK : ResetId = ResetId(1 << 5);
	pub const IOQSPI : ResetId = ResetId(1 << 6);

	pub const JTAG : ResetId = ResetId(1 << 7);

	pub const PADSBANK : ResetId = ResetId(1 << 8);
	pub const PADSQSPI : ResetId = ResetId(1 << 9);

	pub const PIO0 : ResetId = ResetId(1 << 10);
	pub const PIO1 : ResetId = ResetId(1 << 11);

	pub const PLLSYS : ResetId = ResetId(1 << 12);
	pub const PLLUSB : ResetId = ResetId(1 << 13);

	pub const PWM : ResetId = ResetId(1 << 14);

	pub const RTC : ResetId = ResetId(1 << 15);

	pub const SPI0 : ResetId = ResetId(1 << 16);
	pub const SPI1 : ResetId = ResetId(1 << 17);

	pub const SYSCFG : ResetId = ResetId(1 << 18);
	pub const SYSINFO : ResetId = ResetId(1 << 19);

	pub const TBMAN : ResetId = ResetId(1 << 20);

	pub const TIMER : ResetId = ResetId(1 << 21);

	pub const UART0 : ResetId = ResetId(1 << 22);
	pub const UART1 : ResetId = ResetId(1 << 23);

	pub const USBCTRL : ResetId = ResetId(1 << 24);

	/// XORs the ID.
	pub const fn inverse(&self) -> ResetId {
		ResetId(self.0 ^ 0xFFFFFFFF)
	}
}

impl const core::ops::Add<Self> for ResetId {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		ResetId(self.0 | rhs.0)
	}
}

impl const core::ops::BitOr<Self> for ResetId {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output {
		ResetId(self.0 | rhs.0)
	}
}

impl core::ops::AddAssign<Self> for ResetId {
	fn add_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0
	}
}

impl core::ops::BitOrAssign<Self> for ResetId {
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0
	}
}

impl core::convert::From<ResetId> for u32 {
	fn from(x: ResetId) -> u32 {
		x.0
	}
}
