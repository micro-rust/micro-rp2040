//! Clock outputs of the RP2040.


use crate::raw::AtomicRegister;

use micro::Peripheral;
use micro::Register;


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
    #[inline(never)]
    pub(crate) fn init(&mut self) {
        // Configure Reference clock.
        self.reference.init();

        // Configure System clock.
        self.system.init();

        // Configure USB clock.
        self.usb.init();

        // Configure ADC clock.

        // Configure RTC clock.

        // Configure Peripheral clock.
        self.peripheral.init();
    }

    /// Pre initialization sequence.
    pub(crate) fn preinit(&mut self) {
        use crate::features::__XFREQ__;

        // Clear all the frequencies.
        unsafe { crate::sys::CLOCKS.freqs = [0u32; 16] };

        // Enable Watchdog tick.
        let mut watchdog: Peripheral<u32, AtomicRegister<u32>, 12, 0x40058000> = Peripheral::get();
        watchdog[11].write((1 << 9) | (__XFREQ__ / 1_000_000));

        // Disable resuscitation clock.
        let mut resus: Peripheral<u32, AtomicRegister<u32>, 2, 0x40008078> = Peripheral::get();
        resus[0].write(0);

        // Switch reference and system clocks to their defaults.
        let mut reference: Peripheral<u32, AtomicRegister<u32>, 3, 0x40008030> = Peripheral::get();
        reference[0].write(0);

        let mut system: Peripheral<u32, AtomicRegister<u32>, 3, 0x4000803C> = Peripheral::get();
        system[0].write(0);
    }
}
