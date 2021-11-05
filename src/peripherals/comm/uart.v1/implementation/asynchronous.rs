//! Asynchronous implementation of the UART communication peripheral.


use crate::error::SystemError;
use crate::raw::AtomicRegister;
use crate::sync::Syslock;
use crate::sys::{ SystemResource, RESOURCES };
use crate::peripherals::dma::{ DMAChannelTrait, Stream };


use micro::Register;


use super::super::*;


pub struct AsyncUartTX<'a, TX: UartTxPin, CTS: UartCtsPin, DMA: DMAChannelTrait> {
    /// The pins used for communication.
    pub(super) pins: Option<(TX, Option<CTS>)>,

    /// The stream used to transfer the data.
    pub(super) stream: Option<Stream<'a, DMA>>,
}

impl<'a, TX: UartTxPin + Uart0Pin, CTS: UartCtsPin + Uart0Pin, DMA: DMAChannelTrait> AsyncUartTX<'a, TX, CTS, DMA> {
    fn uart0() -> Result<Self, SystemError> {
        match Syslock::acquire() {
            Some(_) => match unsafe { RESOURCES[3] } & (1 << 0) {
                0 => {
                    unsafe { RESOURCES[3] |= 1 << 0 }

                    Ok( Self {
                        pins: None,
                        stream: None,
                    } )
                },
                _ => Err( SystemError::PeripheralNotAvailable ),
            },

            _ => Err( SystemError::NoSystemLock ),
        }
    }
}


impl<'a, TX: UartTxPin + Uart1Pin, CTS: UartCtsPin + Uart1Pin, DMA: DMAChannelTrait> AsyncUartTX<'a, TX, CTS, DMA> {
    fn uart1() -> Result<Self, SystemError> {
        match Syslock::acquire() {
            Some(_) => match unsafe { RESOURCES[3] } & (1 << 1) {
                0 => {
                    unsafe { RESOURCES[3] |= 1 << 1 }

                    Ok( Self {
                        pins: None,
                        stream: None,
                    } )
                },
                _ => Err( SystemError::PeripheralNotAvailable ),
            },

            _ => Err( SystemError::NoSystemLock ),
        }
    }
}


impl<'a, TX: UartTxPin, CTS: UartCtsPin, DMA: DMAChannelTrait> SystemResource for AsyncUartTX<'a, TX, CTS, DMA> {
    /// Configures the UART in TX only mode.
    /// Returns the achieved baudrate.
    pub fn config(&mut self, cfg: UartConfig, baud: u32, tx: TX, cts: Option<CTS>) -> Result<u32, ()> {
        // Configure the TX and CTS pins.
        tx.config();

        if let Some(ref pin) = cts {
            pin.config();
        }

        // Configure TX only.
        let uart = unsafe { &mut *(Self::ADDRESS as *mut [AtomicRegister<u32>; 19]) };

        // Get peripheral clock.
        let freq = unsafe { CLOCKS.freqs[Clock::Peripheral.index()] };

        // Set the baudrate.
        let div = (8 * freq) / baud;

        let (ibrd, fbrd) = match div >> 7 {
            0 => (1, 0),
            65535.. => (65535, 0),
            x => (x, ((div & 0x7F) + 1) / 2),
        };

        // Load IBRD and FBRD.
        uart[ 9].write(ibrd);
        uart[10].write(fbrd);

        // Set the format and FIFOs.
        // Also is used as LCR H write to fix the registers.
        uart[11].write(u32::from(cfg));

        // Get final baudrate.
        let baudrate = (4 * freq) / ((64 * ibrd) + fbrd);

        // Enable TX, CTS and the UART.
        let enable = match cts {
            Some(_) => (1 << 15) | (1 << 8) | 1,
            _ => (1 << 8) | 1,
        };

        uart[12].set(enable);

        // Enable DMA requests.
        uart[18].set(1 << 1);

        Ok( baudrate )
    }
}

