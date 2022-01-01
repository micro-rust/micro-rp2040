//! UART RX Handler.
//! Asynchronous handler of UART RX events.



#[link_section = ".systemdata0.UART0"]
static mut UART0RXHANDLER = RXHandler::new();

#[link_section = ".systemdata1.UART1"]
static mut UART1RXHANDLER = RXHandler::new();


pub struct RXHandler {
    /// Address of the buffer to be received.
    buffer: Option<Pin<&mut [u8]>>,

    /// Index into the buffer.
    idx: usize,

    /// Current status of the transmission.
    status: UARTStatus
}

impl RXHandler {
	/// Static initializer.
	pub const fn new() -> RXHandler {
		RXHandler {
			buffer: None,
			idx: 0,
			status: UARTStatus,
		}
	}

    /// Handles break error.
    /// Break errors DO stop the flow of data.
    pub(super) fn breakerr(&mut self, uart: &mut [AtomicRegister<u32>; 19]) {
        // Clear the IRQ mask.
        uart[17].write(1 << 9);

        // Set internal flag.
        match self.status {
            UARTStatus::Error(_) => (),
            _ => {
                self.status = UARTStatus::Error(UARTError::Break);
                self.buffer = None;
            },
        }
    }

    /// Handles parity error.
    /// Parity errors DO stop the flow of data.
    pub(super) fn parity(&mut self, uart: &mut [AtomicRegister<u32>; 19]) {
        // Clear the IRQ mask.
        uart[17].write(1 << 8);

        // Set internal flag.
        match self.status {
            UARTStatus::Error(_) => (),
            _ => {
                self.status = UARTStatus::Error(UARTError::Parity);
                self.buffer = None;
            },
        }
    }


    /// Handles framing error.
    /// Framing errors do stop the flow of data.
    pub(super) fn framing(&mut self, uart: &mut [AtomicRegister<u32>; 19]) {
        // Clear the IRQ mask.
        uart[17].write(1 << 7);

        // Set internal flag.
        match self.status {
            UARTStatus::Error(_) => (),
            _ => {
                self.status = UARTStatus::Error(UARTError::Framing);
                self.buffer = None;
            },
        }
    }

    /// Handles timeout error.
    /// Timeout errors do stop the flow of data.
    pub(super) fn timeout(&mut self, uart: &mut [AtomicRegister<u32>; 19]) {
        // Clear the IRQ mask.
        uart[17].write(1 << 7);

        // Set internal flag.
        match self.status {
            UARTStatus::Error(_) => (),
            _ => {
                self.status = UARTStatus::Timeout(self.idx);
                self.buffer = None;
            },
        }
    }

    /// Receives the next item from the UART bus.
    pub(super) fn recv(&mut self, uart: &mut [AtomicRegister<u32>; 19]) {
    	// If this handler is 
    }

    /// Receives the next item from the UART bus.
    pub(crate) fn recv(&mut self, uart: &mut [AtomicRegister<u32>; 19]) {

        // Check if there is a buffer to write to.
        match self.data {

            // Write the received data into the buffer.
            Some(buffer) => {

                // Create a mutable index.
                let mut i;

                // Write until FIFO is depleted or the buffer is full.
                for i in self.idx..buffer.len() {
                    // If RX FIFO is empty, break the loop.
                    if (uart[6].read() & (1 << 4)) != 0 { break; }

                    // Read in the data.
                    let word = uart[0].read();

                    // Check for errors.
                    match 0xF00 & word {

                        // No errors -> Write to buffer.
                        0 => {
                            buffer[i] = (word & 0xFF) as u8;
                            continue;
                        },

                        // Check which error happenned.
                        _ => {
                            // Check Framing error.
                            if (word & (1 << 8)) != 0 {
                                self.framing(uart);
                                return;
                            }

                            // Check Parity error.
                            if (word & (1 << 9)) != 0 {
                                self.parity(uart);
                                return
                            }

                            // Check break error.
                            if (word & (1 << 10)) != 0 {
                                self.breakerr(uart);
                                return
                            }

                            // Check overrun error.
                            if (word & (1 << 11)) != 0 {
                                self.overrun(uart);
                                buffer[i] = (word & 0xFF) as u8;
                                continue;
                            }
                        },
                    }
                }

                // Check if the buffer is full.
                if i >= buffer.len() - 1 {
                    // Update status.
                    match self.status {
                        UARTStatus::OngoingOverrun => self.status=  UARTStatus::CompletedOverrun,
                        UARTStatus::Ongoing => self.status=  UARTStatus::Completed,

                        _ => (),
                    }
                }
            },

            // Flush all received data.
            None => {
                while (uart[6].read() & (1 << 4)) == 0 {
                    uart[0].read();
                }
            },
        }

        // Check status.
        match self.status {
            UARTStatus::Completed | UARTStatus::CompletedOverrun => self.buffer = None,
            _ => (),
        }
    }
}
