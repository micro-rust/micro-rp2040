//! Stream module.
//! Abstractions over all types of streams that can be executed in the RP2040.


use crate::dma::DMAChannelTrait;
use crate::error::Error;
use crate::error::DMAError;


mod multi;


pub use self::multi::MultiBlockStream;



/// Common trait for all DMA Streams.
pub trait Stream: Sized {
    /// Creates a stream with the given channel.
    fn create<CH: DMAChannelTrait>(ch: CH) -> Result<Self, Error>;

    /// Validates a stream block for the given stream.
    fn validate<CB: MainControlBlock>(&self, cb: CB) -> Result<(), DMAError>;

    /// Returns the stream transaction count.
    fn ncount(&self) -> u32;

    /// Returns the stream write address.
    fn write_addr(&self) -> u32;

    /// Returns the stream read address.
    fn read_addr(&self) -> u32;
}





/// Control Block for a DMA Channel.
#[repr(C)]
pub struct RawControlBlock<const N: usize>(pub [u32; N]);


impl<const N: usize> RawControlBlock<N> where [(); 4-N]: Sized {
    /// Static initializer.
    pub const fn empty() -> Self {
        Self([0u32; N])
    }
}



pub trait MainControlBlock {
    const OFFSET: usize;
    type Complement: SecondaryControlBlock;
}

pub trait SecondaryControlBlock {
    const OFFSET: usize;
}



/// Full configuration control block.
#[repr(C)]
pub struct CBFull(RawControlBlock<4>);

impl MainControlBlock for CBFull {
    const OFFSET: usize = 0;

    type Complement = ();
}

impl SecondaryControlBlock for () {
    const OFFSET: usize = 0;
}


/// Control only configuration control block.
#[repr(C)]
pub struct CBCtrl(RawControlBlock<1>);

impl MainControlBlock for CBCtrl {
    const OFFSET: usize = 3;

    type Complement = CBReadWriteCount;
}

impl SecondaryControlBlock for CBCtrl {
    const OFFSET: usize = 4;
}

/// Count only configuration control block.
#[repr(C)]
pub struct CBCount(RawControlBlock<1>);

impl MainControlBlock for CBCount {
    const OFFSET: usize = 7;

    type Complement = CBCtrlReadWrite;
}

impl SecondaryControlBlock for CBCount {
    const OFFSET: usize = 2;
}

/// Write only configuration control block.
#[repr(C)]
pub struct CBWrite(RawControlBlock<1>);

impl MainControlBlock for CBWrite {
    const OFFSET: usize = 11;

    type Complement = CBCtrlCountRead;
}

impl SecondaryControlBlock for CBWrite {
    const OFFSET: usize = 1;
}

/// Read only configuration control block.
#[repr(C)]
pub struct CBRead(RawControlBlock<1>);

impl MainControlBlock for CBRead {
    const OFFSET: usize = 15;

    type Complement = CBCtrlWriteCount;
}

impl SecondaryControlBlock for CBRead {
    const OFFSET: usize = 0;
}



/// Count-Control configuration control block.
#[repr(C)]
pub struct CBCountCtrl(RawControlBlock<2>);

impl MainControlBlock for CBCountCtrl {
    const OFFSET: usize = 2;

    type Complement = CBReadWrite;
}

/// Write-Count configuration control block.
#[repr(C)]
pub struct CBWriteCount(RawControlBlock<2>);

impl MainControlBlock for CBWriteCount {
    const OFFSET: usize = 6;

    type Complement = CBCtrlRead;
}

/// Read-Write configuration control block.
#[repr(C)]
pub struct CBReadWrite(RawControlBlock<2>);

impl MainControlBlock for CBReadWrite {
    const OFFSET: usize = 10;

    type Complement = CBCtrlCount;
}

impl SecondaryControlBlock for CBReadWrite {
    const OFFSET: usize = 0;
}

/// Count-Read configuration control block.
#[repr(C)]
pub struct CBCountRead(RawControlBlock<2>);

impl MainControlBlock for CBCountRead {
    const OFFSET: usize = 14;

    type Complement = CBCtrlWrite;
}

/// Control-Read configuration control block.
#[repr(C)]
pub struct CBCtrlRead(RawControlBlock<2>);

impl SecondaryControlBlock for CBCtrlRead {
    const OFFSET: usize = 4;
}

/// Control-Count configuration control block.
#[repr(C)]
pub struct CBCtrlCount(RawControlBlock<2>);

impl SecondaryControlBlock for CBCtrlCount {
    const OFFSET: usize = 8;
}

/// Control-Write configuration control block.
#[repr(C)]
pub struct CBCtrlWrite(RawControlBlock<2>);

impl SecondaryControlBlock for CBCtrlWrite {
    const OFFSET: usize = 12;
}




/// Write-Count-Control configuration control block.
#[repr(C)]
pub struct CBWriteCountCtrl(RawControlBlock<3>);

impl MainControlBlock for CBWriteCountCtrl {
    const OFFSET: usize = 1;

    type Complement = CBRead;
}

/// Read-Write-Count configuration control block.
#[repr(C)]
pub struct CBReadWriteCount(RawControlBlock<3>);

impl MainControlBlock for CBReadWriteCount {
    const OFFSET: usize = 5;

    type Complement = CBCtrl;
}

impl SecondaryControlBlock for CBReadWriteCount {
    const OFFSET: usize = 0;
}

/// Count-Read-Write configuration control block.
#[repr(C)]
pub struct CBCountReadWrite(RawControlBlock<3>);

impl MainControlBlock for CBCountReadWrite {
    const OFFSET: usize = 9;

    type Complement = CBCtrl;
}

/// Write-Count-Read configuration control block.
#[repr(C)]
pub struct CBWriteCountRead(RawControlBlock<3>);

impl MainControlBlock for CBWriteCountRead {
    const OFFSET: usize = 13;

    type Complement = CBCtrl;
}

/// Control-Read-Write configuration control block.
#[repr(C)]
pub struct CBCtrlReadWrite(RawControlBlock<3>);

impl SecondaryControlBlock for CBCtrlReadWrite {
    const OFFSET: usize = 4;
}

/// Control-Count-Read configuration control block.
#[repr(C)]
pub struct CBCtrlCountRead(RawControlBlock<3>);

impl SecondaryControlBlock for CBCtrlCountRead {
    const OFFSET: usize = 8;
}

/// Control-Write-Count configuration control block.
#[repr(C)]
pub struct CBCtrlWriteCount(RawControlBlock<3>);

impl SecondaryControlBlock for CBCtrlWriteCount {
    const OFFSET: usize = 12;
}
