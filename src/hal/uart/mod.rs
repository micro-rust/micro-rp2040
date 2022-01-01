//! UART Communication peripheral module.






/// UART peripheral.
/// It can be used as full duplex or split into RX and TX channels.
pub struct Uart<const N: usize>;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UARTStatus {
    /// Indicates that the UART transmission has completed.
    Completed,

    /// Indicates that the UART transmission has completed but some data was
    /// lost due to overrun in the FIFO.
    CompletedOverrun,

    /// Indicates that the UART transmission is ongoing.
    Ongoing,

    /// Indicates that the UART tranmission is ongoing but some data was
    /// lost due to overrun in the FIFO.
    OngoingOverrun,

    /// Indicates that a timeout ocurred.
    /// Includes the number of data items transmitted.
    Timeout(usize),

    /// Indicates that an error has ocurred and the transmission has stopped.
    Error(UartError),
}
