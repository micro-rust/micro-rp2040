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

    /// Internal method to wnable default clocks.
    #[inline(never)]
    fn setup(addr: u32, int: u32, frac: u32, src: u32) {
        let regs = unsafe { &mut *(addr as *mut [AtomicRegister<u32>; 3]) };

        // Stop the clock cleanly and delay for propagation.
        regs[0].write(0);

        while regs[2].read() == 0 {}

        // Set the divisor.
        regs[1].write((int << 8) | frac);

        // Configure the source and enable.
        regs[0].write((1 << 11) | (src << 5));
    }

    /// Initializes the Clock sources.
    #[inline(never)]
    pub(crate) fn init(&mut self) {
        use crate::sys::{ CLOCKS, clocks::Clock };

        // Configure Reference clock.
        self.reference.init();
        unsafe { CLOCKS.freqs[Clock::Usb.index()] = crate::features::__XFREQ__; }

        // Configure System clock.
        self.system.init();

        // Configure USB clock.
        Self::setup(0x40008054, 1, 0, 0);
        unsafe { CLOCKS.freqs[Clock::Usb.index()] = 48_000_000; }

        // Configure ADC clock.
        Self::setup(0x40008060, 1, 0, 0);
        unsafe { CLOCKS.freqs[Clock::Adc.index()] = 48_000_000; }

        // Configure RTC clock.
        Self::setup(0x4000806C, 1000, 0, 0);
        unsafe { CLOCKS.freqs[Clock::Rtc.index()] = 48_000; }

        // Configure Peripheral clock.
        Self::setup(0x40008048, 1, 0, 0);
        unsafe { CLOCKS.freqs[Clock::Peripheral.index()] = CLOCKS.freqs[Clock::System.index()]; }
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
