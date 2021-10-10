//! Clock outputs of the RP2040.


/// Reference Clock wrapper.
mod reference;

/// System clock wrapper.
mod system;

/// USB Clock wrapper.
mod usb;

/// Peripheral Clock wrapper.
mod peripheral;

pub use self::reference::Reference;
pub use self::system::SystemClock;
pub use self::usb::UsbClock;
pub use self::peripheral::PeripheralClock;



/// Collection of all clock outputs.
/// TODO : Implement AADC, RTC and GP Outputs.
pub struct ClockOutputs {
    /// Reference Clock.
    pub reference: Reference,

    /// Ring Oscillator.
    pub system: SystemClock,

    /// USB Clock.
    pub usb: UsbClock,

    // ADC Clock.
    //pub adc: AdcClock,

    // RTC Clock.
    //pub rtc: RtcClock,

    /// Peripheral Clock.
    pub peripheral : PeripheralClock,
}

impl ClockOutputs {
    /// Static initializer.
    #[inline(always)]
    pub const fn empty() -> ClockOutputs {
        ClockOutputs {
		    reference: Reference::empty(),
		    system: SystemClock::empty(),
		    usb: UsbClock::empty(),
		    //adc: AdcClock::empty(),
		    //rtc: RtcClock::empty(),
		    peripheral : PeripheralClock::empty(),
        }
    }

    /// Initializes the Clock sources.
    pub(crate) fn init(&mut self) {
        // Configure Reference clock.

        // Configure System clock.

        // Configure USB clock.

        // Configure ADC clock.

        // Configure RTC clock.

        // Configure Peripheral clock.
    }

    /// Pre initialization sequence.
    pub(crate) fn preinit(&mut self) {
        // Clear all the frequencies.

        // Enable Watchdog tick.

        // Disable resuscitation clock.

        // Switch reference and system clocks to their defaults.
    }
}
