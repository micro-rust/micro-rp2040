//! ADC errors.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ADCError {
	/// There was an unknown error in the conversion hardware.
	ConversionError,
}