//! PWM peripheral.



pub mod capture;



/// Raw data from PWM ounter.
#[link_section = ".systemdata.high.PWMCNT"]
pub(crate) static mut PWMCNT: [(u16, u16); 8] = [(0u16, 0u16); 8];

/// Raw data from PWM top.
#[link_section = ".systemdata.high.PWMTOP"]
pub(crate) static mut PWMTOP: [u16; 8] = [0u16; 8];


/// PWM clock divider.
#[link_section = ".systemdata.high.PWMDIV"]
pub(crate) static mut PWMDIV: [f32; 8] = [0.0f32; 8];



/// Enumeration of all PWM Channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PWMChannel {
    /// PWM Channel A.
    ChannelA,

    /// PWM Channel B.
    ChannelB,
}


/// PWM Base abstraction.
pub struct PWMBase<const N: usize>;


