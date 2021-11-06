//! Intercore FIFO Mailbox.


use crate::raw::SIORegister;

use micro::Register;


pub struct Mailbox;


impl Mailbox {
    /// Returns `true` if the FIFO is ready to be read.
    #[inline(always)]
    pub fn can_read() -> bool {
        let status = unsafe { & *(0xD0000050 as *const SIORegister<u32>) };
        status.read() & 1 == 1
    }


    /// Returns `true` if the FIFO is ready to be read.
    #[inline(always)]
    pub fn can_write() -> bool {
        let status = unsafe { & *(0xD0000050 as *const SIORegister<u32>) };
        status.read() & (1 << 1) != 0
    }

    /// Sends a message to the other core if there is space available.
    pub fn send(msg: u32) -> Result<(), ()> {
        let status = unsafe { & *(0xD0000050 as *const SIORegister<u32>) };

        match status.read() & (1 << 1) {
            0 => Err(()),
            _ => {
                unsafe { &mut *(0xD0000054 as *mut SIORegister<u32>) }.write(msg);

                Ok(())
            },
        }
    }

    /// Receives a message to the other core if there is data available.
    pub fn recv() -> Result<u32, ()> {
        let status = unsafe { & *(0xD0000050 as *const SIORegister<u32>) };

        match status.read() & (1 << 0) {
            0 => Err(()),
            _ => Ok( unsafe { & *(0xD0000058 as *const SIORegister<u32>) }.read() ),
        }
    }

    /// Waits until there is space in the FIFO and sends a message.
    pub fn send_blocking(msg: u32) {
        while !Self::can_write() { micro::asm::nop() }

        unsafe { &mut *(0xD0000054 as *mut SIORegister<u32>) }.write(msg);

        micro::asm::sev();
    }

    /// Waits until there is data in the FIFO and receives a message.
    pub fn recv_blocking() -> u32 {
        while !Self::can_read() { micro::asm::nop() }

        unsafe { & *(0xD0000058 as *const SIORegister<u32>) }.read()
    }

    /// Drains the FIFO of messages.
    pub fn drain() {
        while Self::can_read() {
            unsafe { & *(0xD0000058 as *const SIORegister<u32>) }.read();
        }
    }
}