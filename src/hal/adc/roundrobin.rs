//! ADC Round Robin module.


use crate::prelude::*;


use super::{ ADC, AnalogChannel, AnalogChannelList };



/// One time ADC blocking measurement.
/// Can be reused.
pub struct ADCRoundRobin;

impl ADCRoundRobin {
    /// Performs continuous measurements in Round Robin.
    pub fn config(&mut self, list: AnalogChannelList, start: AnalogChannel, threshold: u32, shift: bool) {
        const ROUNDROBIN: u32 = (1 << 3) | (1 << 1) | 1;
        const FIFO: u32 = (1 << 3) | 1;

        // Reference to ADC register block.
        let adc = unsafe { &mut *(0x4004C000 as *mut [AtomicRegister<u32>; 9]) };

        // Disable the ADC.
        adc[0].clear(1);

        // Clear any sticky errors.
        adc[0].set(1 << 10);

        // Enable FIFO.
        if shift { adc[2].write( FIFO | (threshold << 24) | (1 << 1) ) }
        else { adc[2].write( FIFO | (threshold << 24) ) }

        // Enable ADC in round robin mode.
        let cfg = (u32::from(list) << 16) | ((start as u32) << 12);
        adc[0].write( ROUNDROBIN | cfg );
    }

    /// Reads all the current data in the FIFO.
    /// Ignores any conversion errors.
    pub fn read<const N: usize>(&mut self) -> [u16; N] {
        // Reference to ADC register block.
        let adc = unsafe { &mut *(0x4004C000 as *mut [AtomicRegister<u32>; 9]) };

        // Create a buffer.
        let mut buffer = [0u16; N];

        for result in &mut buffer {
            // Wait until FIFO not empty.
            while (adc[2].read() & (1 << 8)) != 0 {}

            let data = adc[3].read() as u16;

            if (data >> 15) != 0 { break; }

            *result = data;
        }

        buffer
    }
}

impl core::convert::From<ADC> for ADCRoundRobin {
    #[inline(always)]
    fn from(x: ADC) -> ADCRoundRobin {
        core::mem::forget(x);

        ADCRoundRobin
    }
}

impl Drop for ADCRoundRobin {
    fn drop(&mut self) {
        DropResources::adc();
    }
}
