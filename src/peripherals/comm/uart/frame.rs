//! UART Received Frame.
//! A UART frame contains the data received and additional errors that may occur.
//! The UART can be used without the errors by reading only the incoming bytes.


use micro::drivers::Data;


/// `UartFrame` implements the Data trait.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct UartFrame(u16);


impl UartFrame {
    /// Returns the data from the frame.
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        self.0 as u8
    }

    /// Returns `true` if an error ocurred in this frame.
    #[inline(always)]
    pub fn error(&self) -> bool {
        (self.0 >> 8) != 0
    }

    /// Returns `true` if the frame did not receive a valid stop bit.
    #[inline(always)]
    pub fn frame_error(&self) -> bool {
        (self.0 & (1 << 8)) != 0
    }

    /// Returns `true` if the frame parity was incorrect.
    #[inline(always)]
    pub fn parity_error(&self) -> bool {
        (self.0 & (1 << 9)) != 0
    }

    /// Returns `true` if a break error was encountered.
    #[inline(always)]
    pub fn break_error(&self) -> bool {
        (self.0 & (1 << 10)) != 0
    }

    /// Returns `true` if an overrun error was encountered.
    #[inline(always)]
    pub fn overrun_error(&self) -> bool {
        (self.0 & (1 << 11)) != 0
    }
}


impl Data for UartFrame {
    const SIZE: usize = 2usize;
}