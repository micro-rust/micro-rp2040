//! ADC Pin traits.

use super::*;


/// ADC Function selector.
const FUNCSEL : u32 = 1;


/// Common trait for all ADC pins.
pub trait ADCPin<const N: usize>: PinTrait {
	fn config(&mut self) {


		// Disable digital -> IE = 0, OD = 1
	}
}



impl ADCPin<0> for Gpio<26> {}
impl ADCPin<1> for Gpio<27> {}
impl ADCPin<2> for Gpio<28> {}
impl ADCPin<3> for Gpio<29> {}
