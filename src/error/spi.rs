//! SPI Errors.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPIError {
	UnreachableBaudrate(u32),

	BadPrescaler(u32),

	BadPostscaler((u32, u32)),
}