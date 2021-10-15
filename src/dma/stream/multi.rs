//! Abstraction of a multi block stream.
//! Creates a DMA Stream that feeds control blocks to another DMA Stream.
//! Enables the user to perform complex transactions without CPU intervention.


use crate::dma::DMAChannelTrait;
use crate::error::DMAError;


use core::marker::PhantomData;


use super::{ Stream, MainControlBlock };



pub struct MultiBlockStream<'a, S: Stream, CH: DMAChannelTrait> {
    /// DMA Stream to be controlled.
    slave: S,

    /// Channel to use for the Main transactions.
    mainch: CH,

    _ph: PhantomData<&'a u32>,
}


impl<'a, S: Stream, CH: DMAChannelTrait> MultiBlockStream<'a, S, CH> {
    /// Creates the `MultiBlockStream`.
    pub fn create<CB: MainControlBlock, SCH: DMAChannelTrait>(ch: CH, sch: CH, cb: &'a [CB], init: CB::Complement) -> Result<Self, DMAError> {
        // Validate the control block array.

        // Set the initial control block.

        // Set the master stream.
    }
}

impl<'a, S: Stream, CH: DMAChannelTrait> super::Stream for MultiBlockStream<'a, S, CH> {
    /// Returns the stream transaction count.
    fn ncount(&self) -> u32 {
        self.mainch.ncount()
    }

    /// Returns the stream write address.
    fn write_addr(&self) -> u32 {
        self.mainch.write_addr()
    }

    /// Returns the stream read address.
    fn read_addr(&self) -> u32 {
        self.mainch.read_addr()
    }

    /// Validates a stream block for the given stream.
    fn validate<CB: MainControlBlock>(&self, cb: CB) -> Result<(), DMAError> {
        todo!()
    }
}