//! Common elements of the Rp2040 ADC peripheral.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AnalogChannel {
    Channel0    = 0,
    Channel1    = 1,
    Channel2    = 2,
    Channel3    = 3,
    Temperature = 4,
}




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AnalogChannelList(u8);

#[allow(non_upper_case_globals)]
impl AnalogChannelList {
    pub const Channel0    : AnalogChannelList = AnalogChannelList(0x01);
    pub const Channel1    : AnalogChannelList = AnalogChannelList(0x02);
    pub const Channel2    : AnalogChannelList = AnalogChannelList(0x04);
    pub const Channel3    : AnalogChannelList = AnalogChannelList(0x08);
    pub const Temperature : AnalogChannelList = AnalogChannelList(0x10);
}

impl const core::convert::From<AnalogChannelList> for u32 {
    #[inline(always)]
    fn from(l: AnalogChannelList) -> u32 {
        l.0 as u32
    }
}

impl const core::ops::Add<AnalogChannelList> for AnalogChannelList {
    type Output = AnalogChannelList;

    #[inline(always)]
    fn add(self, rhs: AnalogChannelList) -> Self::Output {
        AnalogChannelList( self.0 | rhs.0 )
    }
}

impl const core::ops::Add<u8> for AnalogChannelList {
    type Output = AnalogChannelList;

    #[inline(always)]
    fn add(self, rhs: u8) -> Self::Output {
        AnalogChannelList( self.0 | (rhs & 0x1F) )
    }
}

impl const core::ops::BitOr<AnalogChannelList> for AnalogChannelList {
    type Output = AnalogChannelList;

    #[inline(always)]
    fn bitor(self, rhs: AnalogChannelList) -> Self::Output {
        AnalogChannelList( self.0 | rhs.0 )
    }
}

impl core::ops::BitOr<u8> for AnalogChannelList {
    type Output = AnalogChannelList;

    #[inline(always)]
    fn bitor(self, rhs: u8) -> Self::Output {
        AnalogChannelList( self.0 | (rhs & 0x1F) )
    }
}



impl core::ops::AddAssign<AnalogChannelList> for AnalogChannelList {
    #[inline(always)]
    fn add_assign(&mut self, rhs: AnalogChannelList) {
        self.0 |= rhs.0;
    }
}

impl core::ops::AddAssign<u8> for AnalogChannelList {
    #[inline(always)]
    fn add_assign(&mut self, rhs: u8) {
        self.0 |= rhs & 0x1F;
    }
}

impl core::ops::BitOrAssign<AnalogChannelList> for AnalogChannelList {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: AnalogChannelList) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitOrAssign<u8> for AnalogChannelList {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: u8) {
        self.0 |= rhs & 0x1F;
    }
}
