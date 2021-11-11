//! UART Communication peripheral module.


use crate::error::SystemError;
use crate::math::UInt32;
use crate::peripherals::pins::uart::*;
use crate::raw::AtomicRegister;
use crate::sys::{ SystemResource, RESOURCES, CLOCKS, clocks::Clock };
use crate::sync::Syslock;


use micro::Register;


pub use self::{
    config::UartConfig, frame::UartFrame,
    handle::UartHandle, tx::UartTx, rx::UartRx,
};


mod config;
mod frame;
mod handle;
mod rx;
mod tx;



#[link_section = ".systembss.UARTHANDLES"]
pub(crate) static mut UARTHANDLES: [UartHandle; 4] = [
    UartHandle::new(), UartHandle::new(),
    UartHandle::new(), UartHandle::new(),
];


/// Type for the UART 0.
pub type Uart0 = Uart<0>;
/// Type for the UART 1.
pub type Uart1 = Uart<0>;


/// UART peripheral.
/// This struct can be used to acquire a UART peripheral.
/// It can be then split into RX and TX channels or a duplex channel.
pub struct Uart<const N: usize>;

impl<const N: usize> Uart<N> {
    /// Configures the UART instance.
    /// Returns the final baudrate reached.
    pub fn config(&mut self, cfg: UartConfig, baud: u32) -> u32 {
        // Set UART Configuration.
        let uart = unsafe { &mut *((0x40034000 + { 0x4000 * N }) as *mut [AtomicRegister<u32>; 19]) };

        // Get peripheral clock.
        let freq = UInt32::new( unsafe { CLOCKS.freqs[Clock::Peripheral.index()] } );

        // Set the baudrate.
        let div = (UInt32::new(8) * freq) / baud;

        let mut ibrd = div >> 7u32;
        let fbrd;

        if ibrd == 0 {
            ibrd = UInt32::new( 1 );
            fbrd = UInt32::new( 0 );
        } else if ibrd >= 65535 {
            ibrd = UInt32::new( 65535 );
            fbrd = UInt32::new( 0 );
        } else {
            fbrd = ((div & 0x7Fu32) + 1u32) / 2u32;
        }

        // Load IBRD and FBRD.
        uart[ 9].write( u32::from( ibrd) );
        uart[10].write( u32::from( fbrd) );

        // Dummy LCR H write.
        uart[11].write(0);

        // Set the format and FIFOs.
        uart[11].write(u32::from(cfg));

        // Get final baudrate.
        let baudrate = (UInt32::new(4) * freq) / ((UInt32::new(64) * ibrd) + fbrd);

        uart[12].set(1);

        // Enable DMA requests.
        uart[18].set(1 << 1);

        u32::from( baudrate )
    }

    /// Splits the given UART into a RX and TX channel.
    pub fn split<RX: UartRxPin<N>, TX: UartTxPin<N>, CTS: UartCtsPin<N>, RTS: UartRtsPin<N>>(&mut self, rx: RX, tx: TX, cts: Option<CTS>, rts: Option<RTS>) -> (UartRx<N, RX, RTS>, UartTx<N, TX, CTS>) {
        // Forget about self to avoid use after free.
        core::mem::forget(self);

        (UartRx::create(rx, rts), UartTx::create(tx, cts))
    }

    /// Creates a RX only UART instance.
    pub fn rxonly<RX: UartRxPin<N>, RTS: UartRtsPin<N>>(&mut self, rx: RX, rts: Option<RTS>) -> UartRx<N, RX, RTS> {
        // Forget about self to avoid use after free.
        core::mem::forget(self);

        UartRx::create(rx, rts)
    }

    /// Creates a TX only UART instance.
    pub fn txonly<TX: UartTxPin<N>, CTS: UartCtsPin<N>>(&mut self, tx: TX, cts: Option<CTS>) -> UartTx<N, TX, CTS> {
        // Forget about self to avoid use after free.
        core::mem::forget(self);

        UartTx::create(tx, cts)
    }
}

impl SystemResource for Uart<0> {
    fn acquire() -> Result<Self, SystemError> {
        match Syslock::acquire() {
            Some(_) => match unsafe { RESOURCES[2] } & (1 << 0) {
                0 => {
                    unsafe { RESOURCES[2] |= 1 << 0 }

                    Ok( Self )
                },
                _ => Err( SystemError::PeripheralNotAvailable ),
            },

            _ => Err( SystemError::NoSystemLock ),
        }
    }
}

impl SystemResource for Uart<1> {
    fn acquire() -> Result<Self, SystemError> {
        match Syslock::acquire() {
            Some(_) => match unsafe { RESOURCES[2] } & (1 << 1) {
                0 => {
                    unsafe { RESOURCES[2] |= 1 << 1 }

                    Ok( Self )
                },
                _ => Err( SystemError::PeripheralNotAvailable ),
            },

            _ => Err( SystemError::NoSystemLock ),
        }
    }
}
