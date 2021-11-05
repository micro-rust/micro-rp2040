//! Uart TX channel.


use crate::peripherals::comm::{ AsyncTx, Sender };
use crate::peripherals::pins::uart::{ UartTxPin, UartCtsPin };
use crate::raw::AtomicRegister;


use micro::Register;


pub struct UartTx<const N: usize, TX: UartTxPin<N>, CTS: UartCtsPin<N>> {
    /// TX Pin.
    tx: TX,

    /// CTS Pin.
    cts: Option<CTS>
}

impl<const N: usize, TX: UartTxPin<N>, CTS: UartCtsPin<N>> UartTx<N, TX, CTS> {
    /// Creates a Uart TX Channel from the UART peripheral token.
    #[inline(always)]
    pub(super) fn create(tx: TX, cts: Option<CTS>) -> Self {
        // Configure the pins.
        match cts {
            Some(ref pin) => pin.config(),
            _ => (),
        }

        tx.config();

        Self { tx, cts }
    }
}

impl<const N: usize, TX: UartTxPin<N>, CTS: UartCtsPin<N>> AsyncTx for UartTx<N, TX, CTS> {
    type Handle = super::UartHandle;

    #[inline(always)]
    fn enable(&mut self) {
        let cr = unsafe { &mut *(self.port() as *mut AtomicRegister<u32>) };

        match self.cts {
            Some(_) => cr.set((1 << 15) | (1 << 8)),
            _ => cr.set( 1 << 8 ),
        }
    }

    #[inline(always)]
    fn disable(&mut self) {
        let cr = unsafe { &mut *(self.port() as *mut AtomicRegister<u32>) };

        match self.cts {
            Some(_) => cr.clear((1 << 15) | (1 << 8)),
            _ => cr.clear( 1 << 8 ),
        }
    }

    #[inline(always)]
    fn port(&self) -> u32 {
        (0x40034000 + { 0x4000 * N }) as u32
    }

    #[inline(always)]
    fn handle<'a>(&self) -> &'a mut Self::Handle {
        unsafe { &mut super::UARTHANDLES[N] }
    }

    #[inline(always)]
    fn dreq(&self) -> u32 {
        20 + (N as u32 * 2)
    }
}

impl<const N: usize, TX: UartTxPin<N>, CTS: UartCtsPin<N>> Sender<u8> for UartTx<N, TX, CTS> {
    #[inline(always)]
    fn config(&mut self) {}
}
