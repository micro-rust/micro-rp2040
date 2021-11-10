//! I2C Configuration.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I2CConfig{
    /// Configuration register.
    pub(super) con: u32,

    /// Baudrate.
    pub(super) baud: u32,
}


impl I2CConfig {
    /// Static initializer.
    pub const fn new() -> Self {
        Self { con: 0, baud: 0 }
    }

    /// Sets Standard Mode.
    #[inline(always)]
    pub fn standard(mut self) -> Self {
        self.con &= !(0x3 << 1);
        self.con |= 0x1 << 1;
        self
    }

    /// Sets Fast Mode or Fast Mode +.
    #[inline(always)]
    pub fn fast(mut self) -> Self {
        self.con &= !(0x3 << 1);
        self.con |= 0x2 << 1;
        self
    }

    /// Sets addressing to 7 bits.
    #[inline(always)]
    pub fn address7bit(mut self) -> Self {
        self.con &= !(0b11 << 3);
        self
    }

    /// Sets addressing to 10 bits.
    #[inline(always)]
    pub fn address10bit(mut self) -> Self {
        self.con |= 0b11 << 3;
        self
    }

    /// Sets the baudrate.
    #[inline(always)]
    pub fn baudrate(mut self, val: u32) -> Self {
        self.baud = val;
        self
    }
}