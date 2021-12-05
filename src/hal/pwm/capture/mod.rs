//! PWM Capture module.


mod handler;

pub use self::handler::PWMCaptureHandler;


/// State machine and handlers for PWM capture.
#[link_section = ".systemdata.high.PWMCAPTREHANDLER"]
pub(crate) static mut PWMCAPTREHANDLER: [PWMCaptureHandler; 8] = [PWMCaptureHandler::new(); 8];


/// Abstraction for PWM input output.
pub struct PWMCapture<const N: usize, PIN: PWMChannelB<N>> {
    pin: PIN,
}

impl<const N: usize, PIN: PWMChannelB<N>> PWMInOut<N, PIN> {
    /// Creates a new PWM Capture abstraction.
    pub fn create(pwm: PWMBase<N>, pin: PIN, cfg: PWMConfig) -> Self {
        use core::mem::forget;

        // Configure the pin.
        pin.config();

        forget(pwm);

        Self { pin }
    }

    /// Configures the duty cycle capture.
    /// The resolution field is interpreted as microseconds.
    pub fn config(&mut self, resolution: u32) {
        // Enable counter only when the PWM B pin is high. 0x1
        pwm[0].write((1 << 4) | (1 << 1));

        // Set up resolution based on SYSFREQ.
        let us = Float32::from(resolution) / 1000000.0;
        let div = Float32::from(sys) * us;

        let idiv = u32::from(div);

        let fdiv = u32::from( (div - Float32::from(idiv)) * 16 );

		pwm[1].write( (idiv << 4) | (fdiv & 0xF) );

        // Configure the interrupt.

    }

    /// Enables the PWM.
    #[inline(always)]
    pub fn enable(&mut self) {
        let pwm = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 5]) };

        pwm[0].set(1);
    }

    /// Disables the PWM.
    #[inline(always)]
    pub fn disable(&mut self) {
        let pwm = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 5]) };

        pwm[0].clear(1);
    }
}
