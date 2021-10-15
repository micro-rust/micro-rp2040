//! Clock sources of the RP2040.


/// System PLL wrapper.
mod pllsys;

/// USB PLL wrapper.
mod pllusb;

/// Crystal Oscillator wrapper.
mod rosc;

/// Crystal Oscillator wrapper.
mod xosc;


use self::pllsys::PllSystem;
use self::pllusb::PllUsb;
use self::rosc::Rosc;
use self::xosc::Xosc;


/// Collection of all clock sources.
/// TODO : Implement GP Inputs.
pub struct ClockSources {
    /// Crystall Oscillator.
    pub xosc: Xosc,

    /// Ring Oscillator.
    pub rosc: Rosc,

    /// System PLL.
    pub pllsys: PllSystem,

    /// USB PLL.
    pub pllusb: PllUsb,
}


impl ClockSources {
    /// Static initializer.
    #[inline(always)]
    pub const fn empty() -> ClockSources {
        ClockSources {
            xosc: Xosc::empty(),
            rosc: Rosc::empty(),

            pllsys: PllSystem::empty(),
            pllusb: PllUsb::empty(),
        }
    }

    /// Initializes the Clock sources.
    #[inline(never)]
    pub(crate) fn init(&mut self) {
        // Disable the System and USB PLL.
        self.pllsys.off();
        self.pllusb.off();

        // Initialize the XOSC.
        self.xosc.init();

        // Initialize the System PLL.
        self.pllsys.init();

        // Initialize USB PLL.
        self.pllusb.init();
    }
}
