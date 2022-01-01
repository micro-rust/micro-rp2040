//! UART handler.




#[link_section = ".systemdata1.UART1"]
static mut UART1RXHANDLER = RXHandler::new();


