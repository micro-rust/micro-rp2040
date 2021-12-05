//! ADC peripheral module.


use crate::prelude::*;
use crate::error::ADCError;


use super::{ ADC, AnalogChannel };



/// One time ADC blocking measurement.
/// Can be reused.
pub struct ADCOneshot;

impl ADCOneshot {
    /// Performs a one shot measurement of the given channel.
    pub fn read(&mut self, ch: AnalogChannel) -> Result<u16, ADCError> {
        const ONESHOT: u32 = (1 << 2) | (1 << 1) | 1;

        // Reference to ADC register block.
        let adc = unsafe { &mut *(0x4004C000 as *mut [AtomicRegister<u32>; 9]) };

        // Disable the ADC.
        adc[0].clear(1);

        // Clear any sticky errors.
        adc[0].set(1 << 10);

        // Disable FIFO.
        adc[2].clear(1);

        // Enable ADC in oneshot mode.
        adc[0].write( ONESHOT | ((ch as u32) << 12) );

        // Wait until conversion completed or error encountered.
        while (adc[0].read() & (0x7 << 8)) == 0 {}

        // Check if there was an error.
        if (adc[0].read() & (0x3 << 9)) != 0 { return Err( ADCError::ConversionError ) }

        Ok( adc[1].read() as u16 )
    }
}

impl core::convert::From<ADC> for ADCOneshot {
    #[inline(always)]
    fn from(x: ADC) -> ADCOneshot {
        core::mem::forget(x);

        ADCOneshot
    }
}

impl Drop for ADCOneshot {
    fn drop(&mut self) {
        DropResources::adc();
    }
}
