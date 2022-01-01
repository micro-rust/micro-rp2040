//! UART Asynchronous handlers.


mod rx;
mod tx;




/// UART 0 IRQ function.
pub(crate) fn uart0() {
    // Reference to the UART peripheral.
    let uart = unsafe { &mut *(0x40034000 as *mut [AtomicRegister<u32>; 19]) };

    // Check if TX or RX IRQ.
    let status = uart[16].read();

    // Check for Break error.
    if (status & (1 << 9)) != 0 {
        unsafe { rx::UART0RX.breakerr(uart); }
    }

    // Check for Parity error.
    if (status & (1 << 8)) != 0 {
        unsafe { rx::UART0RX.parity(uart); }
    }

    // Check for Framing error.
    if (status & (1 << 7)) != 0 {
        unsafe { rx::UART0RX.framing(uart); }
    }

    // Check for Timeout error.
    if (status & (1 << 6)) != 0 {
        unsafe { rx::UART0RX.timeout(uart); }
    }

    // Check if there is data to be received.
    if (status & ((1 << 10) | (1 << 4))) != 0 {
        unsafe { rx::UART0RX.recv(uart); }
    }

    // Check if there is data to be sent.
    if (status & (1 << 5)) != 0 {
        unsafe { tx::UART0TX.send(uart); }
    }
}


/// UART 1 IRQ function.
pub(crate) fn uart1() {
    // Reference to the UART peripheral.
    let uart = unsafe { &mut *(0x40038000 as *mut [AtomicRegister<u32>; 19]) };

    // Check if TX or RX IRQ.
    let status = uart[16].read();

    // Check for Break error.
    if (status & (1 << 9)) != 0 {
        unsafe { rx::UART1RX.breakerr(uart); }
    }

    // Check for Parity error.
    if (status & (1 << 8)) != 0 {
        unsafe { rx::UART1RX.parity(uart); }
    }

    // Check for Framing error.
    if (status & (1 << 7)) != 0 {
        unsafe { rx::UART1RX.framing(uart); }
    }

    // Check for Timeout error.
    if (status & (1 << 6)) != 0 {
        unsafe { rx::UART1RX.timeout(uart); }
    }

    // Check if there is data to be received.
    if (status & ((1 << 10) | (1 << 4))) != 0 {
        unsafe { rx::UART1RX.recv(uart); }
    }

    // Check if there is data to be sent.
    if (status & (1 << 5)) != 0 {
        unsafe { tx::UART1TX.send(uart); }
    }
}
