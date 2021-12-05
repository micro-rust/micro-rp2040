//! PWM Capture handler.



#[repr(C)]
pub struct PWMCaptureHandler(u32);

impl PWMCaptureHandler {
    /// Static initializer.
    pub const fn new() -> Self {
        PWMCaptureHandler(0u32)
    }

    /// Clears this handler to allow for more data to come in.
    #[inline(always)]
    fn clear(&mut self) {
        self.0 &= !0xF0;
    }

    /// Returns the ID of the PWM of this handler.
    #[inline(always)]
    pub fn id(&self) -> usize {
        (self.0 & 0xF) as usize
    }

    /// Returns the captured duty cycle, if it was configured as a duty cycle capture.
    pub fn dutycycle(&mut self) -> f32 {
        // Clear for next capture.
        self.clear();

        // Get ID of the capture.
        let id = self.id();

        // Get top value and count.
        let top = unsafe { super::super::PWMTOP[id] };
        let cnt = unsafe { super::super::PWMCNT[id] };

        f32::from( Float32::from(cnt) / Float32::from(top) )
    }

    /// Returns the period of time the signal was active high.
    pub fn hightime(&mut self) -> f32 {
        // Clear for next capture.
        self.clear();

        // Get ID of the capture.
        let id = self.id();

        // Get count and clock divider.
        let cnt = unsafe { super::super::PWMCNT[id] };
        let div = unsafe { super::super::PWMDIV[id] };

        // Get system frequency.
        let sys = Clocks::system();

        f32::from( Float32::from(cnt) / (Float32::from(sys) / Float32::from(div)) )
    }

    /// Returns the period of time the signal was inactive low.
    pub fn hightime(&mut self) -> f32 {
        // Clear for next capture.
        self.clear();

        // Get ID of the capture.
        let id = self.id();

        // Get top, count and clock divider.
        let top = unsafe { super::super::PWMTOP[id] };
        let cnt = unsafe { super::super::PWMCNT[id] };
        let div = unsafe { super::super::PWMDIV[id] };

        // Get system frequency.
        let sys = Clocks::system();

        f32::from( (Float32::from(top) - Float32::from(cnt)) / (Float32::from(sys) / Float32::from(div)) )
    }
}
