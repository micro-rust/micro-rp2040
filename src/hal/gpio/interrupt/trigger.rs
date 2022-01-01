//! Al possible GPIO triggers.



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigger {
    LevelLow  = 0b0001,
    LevelHigh = 0b0010,
    EdgeLow   = 0b0100,
    EdgeHigh  = 0b1000,
}

impl core::convert::From<Trigger> for u32 {
    fn from(t: Trigger) -> u32 {
        t as u32
    }
}