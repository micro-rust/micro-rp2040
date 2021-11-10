//! `micro-rp2040` UART example.
//! It uses the `ASyncTxInterface`, which is the commoon abstraction for all 
//! communication peripherals that have DMA capabilities.
//! Sending data creates a new `TXHandle` that implements `Future`, allowing
//! for async capabilites.
//! BUG: Currently during setup a corrupted byte might be sent due to the
//! GPIO floating output.


#![no_std]
#![no_main]

use micro_rp2040 as rp2040;

use rp2040::{ main0 };

use rp2040::prelude::*;

use micro::asm::*;


use rp2040::peripherals::comm::{
    AsyncTxInterface, AsyncTx, uart::UartTx, 
};

use rp2040::peripherals::pins::{ Gpio, NULLPIN };

use rp2040::peripherals::dma::{ DMAChannel };



main0!(usermain);


fn usermain() -> ! {
    use rp2040::peripherals::dma::buffer::{ CopyFromRam, SourceBuffer };

    // Get the UART interface to the Picoprobe.
    let mut logger = uart();

    // Prepare hello message.
    let hello: [u8; 52] = [
        b'H', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', b'!', b'\n',
        b'H', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', b'!', b'\n',
        b'H', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', b'!', b'\n',
        b'H', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', b'!', b'\n',
    ];

    let buffer = CopyFromRam::create(&hello).unwrap();

    // Send asynchronously the hello message.
    let handle = logger.send( buffer, false ).unwrap();

    // You can do stuff here.
    // The stream will execute on its own.
    // I, for example, am calculating what's 2 + 2, cause my calculator broke.
    let x = 2;
    let y = 2;

    let z = x + y;

    // Now that we have done our things, wait until the stream completes.
    handle.join();

    loop { nop() }
}


/// Get the default UART logger.
/// With picoprobe, this is UART 0 with pins 0 and 1.
/// Configuration Baudrate: 115200, 8N1.
#[inline(never)]
fn uart() -> AsyncTxInterface<UartTx<0, Gpio<0>, NULLPIN>, DMAChannel<1>> {
    // Create the configuration 8N1.
    const CFG: UartConfig = UartConfig::new().bits8().noparity().stop1();

    // Get the uart.
    let mut uart = Uart::<0>::acquire().unwrap();

    // Configure the UART.
    uart.config( CFG, 115200 );

    // Get the TX pin used.
    let txpin = Gpio::<0>::acquire().unwrap();

    // Get the TX channel of the UART (no CTS).
    let mut txchannel = uart.txonly(txpin, None);
    txchannel.enable();

    // Get the DMA Channel 1.
    let dma = DMAChannel::<1>::acquire();

    // Create the async interface.
    AsyncTxInterface::create(txchannel, dma)
}
