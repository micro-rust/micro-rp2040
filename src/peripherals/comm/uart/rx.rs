//! Uart RX channel.


use crate::peripherals::comm::{ AsyncRx, Receiver };
use crate::peripherals::pins::uart::{ UartRxPin, UartRtsPin };
use crate::raw::AtomicRegister;


use micro::Register;


pub struct UartRx<const N: usize, RX: UartRxPin<N>, RTS: UartRtsPin<N>> {
    /// RX Pin.
    rx: RX,

    /// RTS Pin.
    rts: Option<RTS>
}

impl<const N: usize, RX: UartRxPin<N>, RTS: UartRtsPin<N>> UartRx<N, RX, RTS> {
    /// Creates a Uart RX Channel from the UART peripheral token.
    #[inline(always)]
    pub(super) fn create(rx: RX, rts: Option<RTS>) -> Self {
        // Configure the pins.
        match rts {
            Some(ref pin) => pin.config(),
            _ => (),
        }

        rx.config();

        Self { rx, rts }
    }
}

impl<const N: usize, RX: UartRxPin<N>, RTS: UartRtsPin<N>> AsyncRx for UartRx<N, RX, RTS> {
    type Handle = super::UartHandle;

    #[inline(always)]
    fn enable(&mut self) {
        let cr = unsafe { &mut *(self.port() as *mut AtomicRegister<u32>) };

        match self.rts {
            Some(_) => cr.set((1 << 14) | (1 << 9)),
            _ => cr.set( 1 << 9 ),
        }
    }

    #[inline(always)]
    fn disable(&mut self) {
        let cr = unsafe { &mut *(self.port() as *mut AtomicRegister<u32>) };

        match self.rts {
            Some(_) => cr.clear((1 << 14) | (1 << 9)),
            _ => cr.clear( 1 << 9 ),
        }
    }

    #[inline(always)]
    fn port(&self) -> u32 {
        (0x40034000 + { 0x4000 * N }) as u32
    }

    #[inline(always)]
    fn handle<'a>(&self) -> &'a mut Self::Handle {
        unsafe { &mut super::UARTHANDLES[N + 2] }
    }
}

impl<const N: usize, RX: UartRxPin<N>, RTS: UartRtsPin<N>> Receiver<u8> for UartRx<N, RX, RTS> {
    fn config(&mut self) {}
}
