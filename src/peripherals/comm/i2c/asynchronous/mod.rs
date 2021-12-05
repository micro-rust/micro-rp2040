//! I2C Master module.
//! Controls the communication of an I2C instance in master mode.


mod cfg;


/// Tracks the I2C0 addresses which have an interface associated with it.
#[link_section = ".sysbss0.I2C0ADDR"]
#[used]
static mut I2C0ADDR: [u16; 16] = [0u16; 16];

/// Tracks the I2C1 addresses which have an interface associated with it.
#[link_section = ".sysbss0.I2C1ADDR"]
#[used]
static mut I2C1ADDR: [u16; 16] = [0u16; 16];


/// I2C Masters perform and regulate data communication with an I2C bus.
/// To allow for multiple addresses each I2C instance utilizes one of the 
/// hardware locks available in the Rp2040. I2C0 -> Lock 28. I2C1 -> Lock 29.
pub struct I2CMaster<const N: usize, MDMA: DMAChannelTrait, SDMA: DMAChannelTrait, ADDR: I2CAddress> {
    /// Associated master DMA.
    mdma: MDMA,

    /// Associated slave DMA.
    sdma: SDMA,

    _addr: PhantomData<ADDR>
}


impl<const N: usize, MDMA: DMAChannelTrait, SDMA: DMAChannelTrait, ADDR: I2CAddress> I2CMaster<N, MDMA, SDMA> {
    /// Creates the I2C Master and configures it.
    pub(super) fn create(mdma: MDMA, sdma: SDMA) -> Self {
        // Disable the I2C peripheral.

        // Configure speed, repeated start and address type.

        // Enable the peripheral.

        Self { mdma, sdma, _lock: PhantomData, _addr: PhantomData }
    }
}


impl<MDMA: DMAChannelTrait, SDMA: DMAChannelTrait, ADDR: I2CAddress> I2CMaster<0, MDMA, SDMA, ADDR> {
    /// Sends the given data to the given address.
    pub(self) fn send(&self, addr: ADDR, out: SourceBuffer) -> I2CHandle {
        // Attempt to acquire the lock. If it's in use, an operation is ongoing.
        let lock = match Spinlock28::acquire() {
            Some(l) => l,
            _ => return Err( SystemError::NoSystemLock ),
        };

        // Prepare to send the data.


        // Disable I2C by clearing IC_ENABLE.ENABLE
        // Offset 0x6C. clear 1

        // Write to the IC_CON register to set the maximum speed mode supported (bits 2:1)
        // And the desired speed of the master initiated transfers.
        // 7 or 10 bit addressing.
        // Disable SLAVE bit 6, enable MASTER it 0.

        // block[0].write( (1 << 6) | 1 )
        // if restart { block[0].set( 1 << 5 ) }
        // if addr10bit { block[0].set( 1 << 4 ) }
        // if fast { block[0].set(0x2 << 1) } else { block[0].set(0x1 << 1) }


        // Write to the IC_TAR register the address of the device to be addressed.
        // Also indicates whether a Genral Call or a START BYTE is going ot be performed.

        // Enable the I2C.

        // Write transfer direction and data to be sent to the IC_DATA_CMD register

        // This step generates the START condition and the address byte.

        // Once there is data in the TX FIFO, it starts.
    }
}


impl<MDMA: DMAChannelTrait, SDMA: DMAChannelTrait, LOCK: SpinlockTrait, ADDR: I2CAddress> I2CMaster<0, MDMA, SDMA, LOCK, ADDR> {
    /// Spawns an I2C Master interface to communicates with the given address.
    /// Returns an error if the interface is already taken.
    pub fn interface(&self, addr: ADDR) -> Result<I2CMasterInterface, SystemError> {
        // Try to acquire the lock.
        let lock = match LOCK::acquire() {
            Some(l) => l,
            _ => return Err( SystemError::NoSystemLock ),
        };

        // Index of first empty space.
        let mut idx = None;

        // Transform the address into a u16 to match.
        let addr = u16::from(addr);

        // Check if the interface is taken.
        for (i, iface) in unsafe { (&mut I2C0ADDR).enumerate() } {
            if iface == addr { return Err( SystemError::PeripheralNotAvailable ) }
            if iface == 0 { idx = Some(i) }
        }

        match idx {
            // If there is an empty space.
            Some(i) => {
                unsafe { I2C0ADDR[i] = addr; }
                Ok( I2CMasterInterface::create(self) )
            },

            _ => Err( SystemError::PeripheralNotAvailable ),
        }
    }
}