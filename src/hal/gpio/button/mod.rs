//! Abstraction to use a GPIO pin input as a button.


pub struct GPIOButton<const N: u8>;

impl<const N: u8> GPIOButton<N> {
    /// Configures the given GPIO pin as a button.
    /// The callback is executed on each press.
    pub fn create(pin: Gpio<N>) -> GPIOButton<N> {
        // Create the interrupt.
        let mut int: GPIOInterrupt<N> = GPIOInterrupt::create( Gpio::raw() );
        core::mem::forget(int);

        Self
    }

    /// Configures the button to execute the given callbacks when the button is pressed.
    pub fn on_press(&mut self, callback: [Option<fn()>; 2]) {
        // Create the interrupt.
        let mut int: GPIOInterrupt<N> = GPIOInterrupt::raw( Gpio::raw() );

        // Create the config.
        let mut config = [None, None];

        if let Some(f) = callback[0] {
            config[0] = Some((GPIOTrigger::EdgeHigh, f));
        }

        if let Some(f) = callback[1] {
            config[1] = Some((GPIOTrigger::EdgeHigh, f));
        }

        int.forever(config);
    }

    /// Configures the button to execute the given callbacks when the button is released.
    pub fn on_release(&mut self, callback: [Option<fn()>; 2]) {
        // Create the interrupt.
        let mut int: GPIOInterrupt<N> = GPIOInterrupt::raw( Gpio::raw() );

        // Create the config.
        let mut config = [None, None];

        if let Some(f) = callback[0] {
            config[0] = Some((GPIOTrigger::EdgeLow, f));
        }

        if let Some(f) = callback[1] {
            config[1] = Some((GPIOTrigger::EdgeLow, f));
        }

        int.forever(config);
    }

    /// Configures the button to execute the given callbacks when the button enters pressed state.
    pub fn pressed(&mut self, callback: [Option<fn()>; 2]) {
        // Create the interrupt.
        let mut int: GPIOInterrupt<N> = GPIOInterrupt::raw( Gpio::raw() );

        // Create the config.
        let mut config = [None, None];

        if let Some(f) = callback[0] {
            config[0] = Some((GPIOTrigger::LevelHigh, f));
        }

        if let Some(f) = callback[1] {
            config[1] = Some((GPIOTrigger::LevelHigh, f));
        }

        int.forever(config);
    }
}

impl<const N: u8> Release for GPIOInterrupt<N> {
    fn release(&mut self) {
        GPIOInterrupt::raw( Gpio::raw() ).release();

        core::mem::forget(self);
    }
}

impl<const N: u8> Drop for GPIOInterrupt<N> {
    fn drop(&mut self) {
        GPIOInterrupt::raw( Gpio::raw() ).release();
    }
}
