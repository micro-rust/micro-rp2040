//! ADC peripheral module.


use crate::prelude::*;



mod common;
mod oneshot;
mod roundrobin;


pub use self::common::*;
pub use self::oneshot::ADCOneshot;
pub use self::roundrobin::ADCRoundRobin;



pub struct ADC;


impl ADC {
    /// Creates a One shot conversion mode from the ADC.
    #[inline(always)]
    pub fn oneshot(&mut self) -> ADCOneshot {
        core::mem::forget(self);

        ADCOneshot
    }

    /// Creates a Round Robin conversion mode from the ADC.
    #[inline(always)]
    pub fn roundrobin(&mut self) -> ADCRoundRobin {
        core::mem::forget(self);

        ADCRoundRobin
    }
}


impl SystemResource for ADC {
    fn acquire() -> Result<Self, SystemError> {
        match Syslock::acquire() {
            Ok(_) => match Resources::adc() {
                Some(_) => Ok( Self ),
                _ => Err( SystemError::PeripheralNotAvailable ),
            },

            _ => Err( SystemError::NoSystemLock )
        }
    }

    fn release(&mut self) {
        DropResources::adc();

        core::mem::forget(self);
    }
}


impl Drop for ADC {
    fn drop(&mut self) {
        DropResources::adc();
    }
}



impl core::convert::From<ADCOneshot> for ADC {
    #[inline(always)]
    fn from(x: ADCOneshot) -> ADC {
        core::mem::forget(x);

        ADC
    }
}

impl core::convert::From<ADCRoundRobin> for ADC {
    #[inline(always)]
    fn from(x: ADCRoundRobin) -> ADC {
        core::mem::forget(x);

        ADC
    }
}
