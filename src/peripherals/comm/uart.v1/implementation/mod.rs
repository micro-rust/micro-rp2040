//! Contains all the implementations of the UART communication peripheral.



mod asynchronous;



pub use self::asynchronous::{ AsyncUartRX, AsyncUartTx, AsyncUartDuplex };