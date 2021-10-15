//! Aynchronous UART Peripheral.
//! Uses DMA to digest data without user input.


use crate::pins::uart::*;
use crate::sys::{ RESOURCES, SystemResource };



pub struct AsyncUartTX<TX: UartTxPin, CTS: UartCtsPin> {
    tx: TX,
    cts: Option<CTS>,
}

pub struct AsyncUartRX<RX: UartRxPin, RTS: UartRtsPin> {
    rx: RX,
    rts: Option<RTS>,
}

pub struct AsyncUart<RX: UartRxPin, TX: UartTxPin, CTS: UartCtsPin, RTS: UartRtsPin> {
    /// Transmit channel.
    tx: Option<AsyncUartTX<TX, CTS>>,

    /// DMA Transmit Channel.
    txdma: Option<DMAChannel>,

    /// Receive channel.
    rx: Option<AsyncUartRX<RX, RTS>>,

    /// DMA Receive Channel.
    rxdma: Option<DMAChannel>,
}


impl<RX: UartRxPin + Uart0Pin, TX: UartTxPin + Uart0Pin, CTS: UartCtsPin + Uart0Pin, RTS: UartRtsPin + Uart0Pin> SystemResource for AsyncUart<RX, TX, CTS, RTS> {
    const LOCKNUM: usize = 0;
    const LOCKOFF: u8 = 0;

    fn acquire() -> Result<Self, ()> {
        match Syslock::acquire() {
            Some(_) => match unsafe { RESOURCES[Self::LOCKNUM] & (1 << Self::LOCKOFF) } {
                0 => unsafe {
                    RESOURCES[Self::LOCKNUM] |= 1 << Self::LOCKOFF;

                    Ok(Self { tx: None, rx: None, })
                },

                _ => Err(()),
            },

            _ => Err(()),
        }
    }
}

impl<RX: UartRxPin + Uart1Pin, TX: UartTxPin + Uart1Pin, CTS: UartCtsPin + Uart1Pin, RTS: UartRtsPin + Uart1Pin> SystemResource for AsyncUart<RX, TX, CTS, RTS> {
    const LOCKNUM: usize = 0;
    const LOCKOFF: u8 = 1;

    fn acquire() -> Result<Self, ()> {
        match Syslock::acquire() {
            Some(_) => match unsafe { RESOURCES[Self::LOCKNUM] & (1 << Self::LOCKOFF) } {
                0 => unsafe {
                    RESOURCES[Self::LOCKNUM] |= 1 << Self::LOCKOFF;

                    Ok(Self { tx: None, rx: None, })
                },

                _ => Err(()),
            },

            _ => Err(()),
        }
    }
}


impl<RX: Uart0Pin + UartRxPin, RTS: Uart0Pin + UartRtsPin> AsyncUart<RX, NULLPIN, CTS, NULLPIN> {
    fn rxonly(&mut self, rx: RX, rts: Option<RTS>, rxdma: DMAChannel) -> Result<Self, ()> {
        // Check not initialized.
        if self.rx.is_some() || self.tx.is_some() { return Err(()); }

        // Configure the RX and RTS pins.
        if let Some(ref pin) = rts {
            pin.config();
        }

        rx.config();

        // Configure RX only.
        let baudrate = Self::__rxonly__(0x40034000, cfg, baud);

        Ok( (Self { uart: unsafe { &mut *(0x40034000 as *mut _) }, rx: Some(rx), tx: None, cts: None, rts, }, baudrate) )

    }
}