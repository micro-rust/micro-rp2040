//! UART implementations.


mod asynchronous;
mod blocking;



pub use self::asynchronous::UartAsync;
pub use self::blocking::Uart;
