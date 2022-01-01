//! # Allocation module.
//! 
//! This is not a TRUE allocator, but serves as a buffer tracker.
//! This module helps the user to reserve buffers outside the stack without
//! worrying about manually tracking them and making sure they don't overlap.
//! 
//! The allocator is mainly designed for high data density memory buffering.
//! This allocator is designed to push as much data as possible into SRAM
//! at the expense of time spent looking for free memory.
//! 
//! Overall performance in tests is `O(n)` where `n` is the current amount of
//! data allocated. Bigger buffers have better performance as there are less
//! checks to be made.
//! 
//! Once most memory has been allocated, small objects can still be allocated
//! in the residual sections between objects, at an increased computation cost.
//! 
//! If the user wants to use a true allocator, simply reserve some buffers
//! and point the allocator to those pages.
//! 
//! This allocator has 1 kB of memory overhead, leaving 255 kB for the user (99.6%).
//! The granularity of the allocator is 32 bytes (8 words).
//! 



#![feature(const_mut_refs)]


/// Allocates memory buffers in chunks of 1 kBytes (1024).
#[derive(Debug, Clone, Copy)]
pub struct Allocator {
    /// Bit tracking of the memory regions.
    l0: [u32; 256],
}

impl Allocator {
    /// Static initializer.
    pub const fn new() -> Allocator {
        Allocator { l0: [0u32; 256] }
    }

    /// Attempts to allocate a buffer.
    /// Cannot allocate more than 2048 bytes at once.
    /// The buffers can be merged later.
    pub fn allocate(&mut self, size: u32) -> Option<u32> {
        // If size is 0, return.
        if size == 0 { return None; }

        // Calculate how many 32 byte blocks are needed.
        let n = match size & 0x1F {
            0 => size >> 5,
            _ => (size >> 5) + 1,
        };

        // Loop variable.
        let mut i = 0;

        // Check in each block if it can allocate the block.
        loop {
            if i >= 256 { break; }

            match self.allocateblock(n, i) {
                Some(addr) => {
                    return Some( addr );
                },
                None => (),
            }

            i += 1;
        }

        // If no block can allocate it raw, see if mixing blocks can do it.
        loop {
            if i >= 255 { break; }

            // Tail count.
            let tail = self.tailcount(i);

            // Head count.
            let head = self.headcount(i);

            // Check if it's enough to allocate the object.
            if n <= (tail + head) {
                // Allocate the tail.
                self.tailalloc(tail, i);

                // Allocate the head.
                self.headalloc(n - tail, i);

                // Return the address.
                let addr = 0x20000000 + (i as u32 * 1024) + ((32 - tail) * 32);

                return Some( addr );
            }

            i += 1;
        }

        None
    }

    /// Deallocates the given object.
    pub fn dealloc<T: Sized>(&mut self, ptr: *mut T) {
        self.deallocate(ptr as u32, core::mem::size_of::<T>() as u32)
    }

    /// Deallocates a given size from an address.
    fn deallocate(&mut self, addr: u32, size: u32) {
        // Calculate how many 32 byte blocks are needed.
        let n = match size & 0x1F {
            0 => size >> 5,
            _ => (size >> 5) + 1,
        };

        // Calculate the base address.
        let idx = (addr as usize - 0x20000000) >> 10;

        match n {
            0..=32 => {
                // Create the mask for the first tracker.
                let mask = match n - 32 {
                    32 => 0xFFFFFFFF,
                    m => (1 << m) - 1,
                };

                self.l0[idx] &= !mask;
            },

            _ => {
                // Create the mask for the second tracker.
                let mask = match n - 32 {
                    32 => 0xFFFFFFFF,
                    m => (1 << m) - 1,
                };

                // Clear the flags.
                self.l0[idx+0] = 0x00000000;
                self.l0[idx+1] &= !mask;
            },
        }
    }

    /// Checks the given L1 tracker and returns the possible configuration data.
    fn allocateblock(&mut self, n: u32, idx: usize) -> Option<u32> {
        // Early return.
        if self.l0[idx] == 0xFFFFFFFF { return None; }

        // Create the mask.
        let mask = match n {
            32 => match self.l0[idx] {
                0 => {
                    // Set flags.
                    self.l0[idx] = 0xFFFFFFFF;

                    // Create address.
                    let addr = 0x20000000 + (idx as u32 * 1024);

                    return Some(addr);
                },
                _ => return None,
            },
            _ => (1 << n) - 1,
        };
        // Loop variable.
        let mut k = 0u32;

        // Get the L1 tracker.
        let tracker = self.l0[idx];

        // Check all internal blocks.
        loop {
            if k >= 32 - n { break; }

            match tracker & (mask << k) {
                0 => {
                    // Set the flags.
                    self.l0[idx] |= mask << k;

                    // Create address.
                    let addr = 0x20000000 + (idx as u32 * 1024) + (k * 32);

                    return Some(addr);
                },

                _ => (),
            }

            k += 1;
        }

        None
    }

    /// Returns the amount of free space in the allocator.
    pub fn free(&self) -> u32 {
        // Get counter.
        let mut c = 0;

        for i in 0..256 {
            c += self.freeblock(i);
        }

        c
    }

    /// Returns the amount of free bytes in this block.
    fn freeblock(&self, idx: usize) -> u32 {
        // Counter.
        let mut c = 0;

        // Get the tracker.
        let tracker = self.l0[idx];

        for k in 0..32 {
            if (tracker & (1 << k)) == 0 {
                c += 1;
            }
        }

        c * 32
    }

    /// Returns the number of free blocks at the start of an L1 tracker.
    fn headalloc(&mut self, n: u32, idx: usize) {
        // Create the mask.
        let mask = match n {
            32 => 0xFFFFFFFF,
            _ => (1 << n) - 1,
        };

        // Check that the mask does not overlap.
        match self.l0[idx] & mask {
            0 => self.l0[idx] |= mask,
            _ => {
                panic!("Could not HEAD allocate, overlap of regions.")
            },
        }

    }

    /// Returns the number of free blocks at the end of an L1 tracker.
    fn tailalloc(&mut self, n: u32, idx: usize) {
        // Create the mask.
        let mask = match n {
            32 => 0xFFFFFFFF,
            _ => ((1 << n) - 1) << (32 - n),
        };

        // Check that the mask does not overlap.
        match self.l0[idx] & mask {
            0 => self.l0[idx] |= mask,
            _ => panic!("Could not TAIL allocate, overlap of regions."),
        }
    }

    /// Returns the number of free blocks at the start of an L1 tracker.
    fn headcount(&mut self, idx: usize) -> u32 {
        // Counter.
        let mut c = 0;

        // Get the tracker.
        let tracker = self.l0[idx];

        // Check internal blocks.
        for k in 0..32 {
            if (tracker & (1 << k)) != 0 {
                return c;
            }

            c += 1;
        }

        c
    }

    /// Returns the number of free blocks at the end of an L1 tracker.
    fn tailcount(&mut self, idx: usize) -> u32 {
        // Counter.
        let mut c = 0;

        // Get the tracker.
        let tracker = self.l0[idx];

        // Check internal blocks.
        for k in 0..32 {
            if (tracker & (1 << (31 - k))) != 0 {
                return c;
            }

            c += 1;
        }

        c
    }
}
