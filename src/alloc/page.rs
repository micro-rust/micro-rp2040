//! Allocator for big objects (128 bytes or more).


use crate::math::UInt32;


/// A Big Page controls the memory usage of a 4096 byte block.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Page<const BLOCKSIZE: u32> {
    /// Address of the Page.
    addr: u32,

    /// Marks the used blocks of the Page.
    /// Each block has 128 bytes.
    used: u32,
}

impl<const BLOCKSIZE: u32> Page<BLOCKSIZE> {
    /// Creates a new Big Page to control the given block.
    #[inline(always)]
    pub(super) const fn create(addr: u32) -> Self {
        Self { addr, used: 0u32, }
    }

    /// Returns `true` if the Big Oage controls the address given.
    #[inline(always)]
    pub(super) fn contains(&self, ptr: *mut u8) -> bool {
        (ptr as u32 - self.addr) < BLOCKSIZE
    }

    /// Returns `true` if the page is empty.
    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.used == 0
    }

    /// Returns the address of the page.
    #[inline(always)]
    pub fn addr(&self) -> u32 {
        self.addr
    }

    /// Reserves the whole page.
    #[inline(always)]
    pub fn reserve(&mut self) {
        self.used = 0xFFFFFFFF;
    }

    /// Configures this page as the page pointing to region 0.
    #[inline(always)]
    pub fn region0(&mut self) {
        self.used |= 0xFF;
    }

    /// Initializes an array of Pages.
    pub fn array<'a>(addr: *mut u8, mut region: u32) -> &'a mut [Self; 32] {
        use core::ptr::write_volatile as write;

        // Change to a u32 pointer.
        let mut addr = addr as *mut Self;

        // Zero out the whole array region (128 bytes).
        for _ in 0..32 {
            unsafe {
                write(addr, Self { addr: region, used: 0 });
                addr = addr.offset(1);
                region = region + (BLOCKSIZE * 32);
            }
        }

        unsafe { &mut *(addr as *mut _) }
    }

    /// Assume that no object is gonna need over 4096 bytes alignment.
    pub(super) fn alloc<const N: u32>(&mut self, size: u32) -> Option<*mut u8> {
        // Get the number of blocks the objects needs.
        let mut n = UInt32::new(size) / BLOCKSIZE;
        if (n % BLOCKSIZE) != 0 { n += 1 }

        // Create the mask of the blocks.
        let mask = (1u32 << n) - 1;

        for i in 0..u32::from(31 - n) {
            if (self.used & (mask << i)) == 0 {
                // Mark as used.
                self.used |= mask << i;

                // Returns the address.
                let ptr = self.addr + (i * BLOCKSIZE);

                return Some( ptr as *mut u8 );
            }
        }

        None
    }

    /// Deallocates the given data.
    /// Returns `true` if this page can be deallocated.
    pub(super) fn dealloc(&mut self, ptr: *mut u8, size: u32) -> bool {
        // Get the number of blocks the objects needs.
        let mut n = UInt32::new(size) / BLOCKSIZE;
        if (n % BLOCKSIZE) != 0 { n += 1 }

        // Create the mask of the blocks.
        let mask = (1u32 << n) - 1;

        // Calculate the delta in blocks.
        let delta = UInt32::new(ptr as u32 - self.addr) / BLOCKSIZE;

        // Mark as clear.
        self.used &= !(mask << delta);

        self.used == 0
    }
}
