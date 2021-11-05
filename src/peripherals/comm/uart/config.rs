//! UART Configuration structure.
//! Const initialized structure that can configure a UART instance.


#[derive(Debug, Clone, Copy)]
pub struct UartConfig(pub(crate) u32);

impl UartConfig {
    /// Creates an empty configuration.
    /// By default, this configuration enabled the FIFOs.
    #[inline(always)]
    pub const fn new() -> Self {
        Self(1 << 4)
    }

    /// Sets the frame size to 5 bits.
    #[inline(always)]
    pub const fn bits5(self) -> Self {
        Self( self.0 & !(0x3 << 5) )
    }

    /// Sets the frame size to 6 bits.
    #[inline(always)]
    pub const fn bits6(self) -> Self {
        Self( (self.0 & !(0x3 << 5)) | (0x1 << 5) )
    }

    /// Sets the frame size to 7 bits.
    #[inline(always)]
    pub const fn bits7(self) -> Self {
        Self( (self.0 & !(0x3 << 5)) | (0x2 << 5) )
    }

    /// Sets the frame size to 8 bits.
    #[inline(always)]
    pub const fn bits8(self) -> Self {
        Self( self.0 | (0x3 << 5) )
    }

    /// Sets even parity checking.
    #[inline(always)]
    pub const fn even(self) -> Self {
        Self( self.0 | (1 << 2) | (1 << 1) )
    }

    /// Disables parity checking.
    #[inline(always)]
    pub const fn noparity(self) -> Self {
        Self( self.0 & !(1 << 1) )
    }

    /// Sets odd parity checking.
    #[inline(always)]
    pub const fn odd(self) -> Self {
        Self( (self.0 & !(1 << 2)) | (1 << 1) )
    }

    /// Sets the number of stop bits to 1.
    #[inline(always)]
    pub const fn stop1(self) -> Self {
        Self( self.0 & !(1 << 3) )
    }

    /// Sets the number of stop bits to 2.
    #[inline(always)]
    pub const fn stop2(self) -> Self {
        Self( self.0 | (1 << 3) )
    }
}
