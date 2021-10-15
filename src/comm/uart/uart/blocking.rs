//! Blocking UART implementation.


use crate::comm::uart::*;
use crate::pins::uart::*;
use crate::power::{ RESET, ResetId };
use crate::raw::AtomicRegister;
use crate::sys::Resource;
use crate::time::{ CLOCKS, clocks::Clock };


use micro::Register;



/// Blocking UART peripheral.
pub struct Uart<RX: UartRxPin, TX: UartTxPin, CTS: UartCtsPin, RTS: UartRtsPin> {
    /// Reference to the UART register block.
    pub(super) uart: &'static mut [AtomicRegister<u32>; 19],

    /// RX pin.
    rx: Option<RX>,

    /// TX pin.
    tx: Option<TX>,

    /// CTS pin.
    cts: Option<CTS>,

    /// RTS pin.
    rts: Option<RTS>,
}



impl<RX: UartRxPin + Uart0Pin, TX: UartTxPin + Uart0Pin, CTS: UartCtsPin + Uart0Pin, RTS: UartRtsPin + Uart0Pin> Uart<RX, TX, CTS, RTS> {
    /// Internal method to acquire the UART 0 peripheral.
    fn acquire() -> Result<(), ()> {
        match Resource::UART0.reserve() {
            Ok(_) => match unsafe { CLOCKS.outputs.peripheral.freeze() } {
                Some(_) => Ok( RESET.cycle(ResetId::UART0) ),
                _ => Err(())
            },

            _ => Err(()),
        }
    }

    /// Attempts to acquire the UART 0 peripheral in RX only mode.
    pub fn rxonly(cfg: UartConfig, baud: u32, rx: RX, rts: Option<RTS>) -> Result<(Self, u32), ()> {
        // Acquire the resource.
        Self::acquire()?;

        // Configure the RX and RTS pins.
        if let Some(ref pin) = rts {
            pin.config();
        }

        rx.config();

        // Configure RX only.
        let baudrate = Self::__rxonly__(0x40034000, cfg, baud);

        Ok( (Self { uart: unsafe { &mut *(0x40034000 as *mut _) }, rx: Some(rx), tx: None, cts: None, rts, }, baudrate) )
    }

    /// Attempts to acquire the UART 0 peripheral in TX only mode.
    pub fn txonly(cfg: UartConfig, baud: u32, tx: TX, cts: Option<CTS>) -> Result<(Self, u32), ()> {
        // Acquire the resource.
        Self::acquire()?;

        // Configure the RX and RTS pins.
        if let Some(ref pin) = cts {
            pin.config();
        }

        tx.config();

        // Configure RX only.
        let baudrate = Self::__txonly__(0x40034000, cfg, baud);

        Ok( (Self { uart: unsafe { &mut *(0x40034000 as *mut _) }, rx: None, tx: Some(tx), cts, rts: None, }, baudrate) )
    }

    /// Attempts to acquire the UART 0 peripheral in duplex mode.
    pub fn duplex(cfg: UartConfig, baud: u32, rx: RX, tx: TX, cts: Option<CTS>, rts: Option<RTS>) -> Result<(Self, u32), ()> {
        // Acquire the resource.
        Self::acquire()?;

        // Configure the RX and RTS pins.
        if let Some(ref pin) = rts {
            pin.config();
        }

        if let Some(ref pin) = cts {
            pin.config();
        }

        rx.config();
        tx.config();

        // Configure duplex only.
        let baudrate = Self::__txonly__(0x40034000, cfg, baud);

        Ok( (Self { uart: unsafe { &mut *(0x40034000 as *mut _) }, rx: Some(rx), tx: Some(tx), cts, rts, }, baudrate) )
    }
}



impl<RX: UartRxPin, TX: UartTxPin, CTS: UartCtsPin, RTS: UartRtsPin> Uart<RX, TX, CTS, RTS> {
    /// Configures the baudrate of the UART block.
    fn __baudrate__(uart: &'static mut [AtomicRegister<u32>; 19], baud: u32) -> u32 {
        // Get peripheral clock (already frozen).
        let peri = unsafe { CLOCKS.freqs[Clock::Peripheral.index()] };

        // Calculate divisor.
        let div = (8 * peri) / baud;

        let mut ibrd = div >> 7;
        let mut fbrd = 0;

        if ibrd == 0 {
            ibrd = 1;
        } else if ibrd >= 65535 {
            ibrd = 65535;
        } else {
            fbrd = ((div & 0x7F) + 1) / 2;
        }

        // Load IBRD and FBRD.
        uart[ 9].write(ibrd);
        uart[10].write(fbrd);

        // Dummy LCR H write.
        uart[11].write(0);

        (4 * peri) / ((64 * ibrd) + fbrd)
    }

    /// Configures the pins given.
    fn __pins__(rx: Option<RX>, tx: Option<TX>, cts: Option<CTS>, rts: Option<RTS>) {
        if let Some(pin) = rx {
            pin.config();
        }

        if let Some(pin) = tx {
            pin.config();
        }

        if let Some(pin) = cts {
            pin.config();
        }

        if let Some(pin) = rts {
            pin.config();
        }
    }

    /// Configures the given UART block as RX only.
    fn __rxonly__(uart: u32, cfg: UartConfig, baud: u32) -> u32 {
        Self::__config__(uart, cfg, baud, (1 << 14) | (1 << 9) | 1)
    }

    /// Configures the given UART block as TX only.
    fn __txonly__(uart: u32, cfg: UartConfig, baud: u32) -> u32 {
        Self::__config__(uart, cfg, baud, (1 << 15) | (1 << 8) | 1)
    }

    /// Configures the given UART block as duplex.
    fn __duplex__(uart: u32, cfg: UartConfig, baud: u32) -> u32 {
        Self::__config__(uart, cfg, baud, (1 << 15) | (1 << 14) | (1 << 9) | (1 << 8) | 1)
    }

    fn __config__(uart: u32, cfg: UartConfig, baud: u32, enable: u32) -> u32 {
        // Reference to the register block.
        let uart: &'static mut [AtomicRegister<u32>; 19] = unsafe { &mut *(uart as *mut _) };

        // Get peripheral clock (already frozen).
        let peri = unsafe { CLOCKS.freqs[Clock::Peripheral.index()] };

        // Set the baudrate.
        let div = (8 * peri) / baud;

        let (ibrd, fbrd) = match div >> 7 {
            0 => (1, 0),
            65535.. => (65535, 0),
            x => (x, ((div & 0x7F) + 1) / 2 ),
        };

        // Load IBRD and FBRD.
        uart[ 9].write(ibrd);
        uart[10].write(fbrd);

        // Dummy LCR H write.
        uart[11].write(0);

        let baudrate = (4 * peri) / ((64 * ibrd) + fbrd);



        // Set the format and FIFOs.
        uart[11].write(cfg.0);

        // Enable RX, RTS and the UART.
        uart[12].set(enable);

        // Enable DMA requests.
        uart[18].set((1 << 1) | 1);

        baudrate
    }
}




impl<RX: UartRxPin, TX: UartTxPin, CTS: UartCtsPin, RTS: UartRtsPin> Drop for Uart<RX, TX, CTS, RTS> {
    fn drop(&mut self) {
        // Reset the peripheral.
        // Return the Peripheral resource.
        // Unfreeze the clock.
        // Release the pins.
    }
}

