//! UART implementations.


mod asynchronous;
mod blocking;



pub use self::blocking::Uart;
//pub use self::asynchronous::UartAsync;
