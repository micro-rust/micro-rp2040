//! Experimental SLAB allocator for the RP2040.
//! This secondary allocator is used for small objects with high allocation and
//! deallocation rates.
//! 
//! It has worse performance than the Buffer Allocator for bigger objects but
//! higher for its intended use.



/// A Slab is the base component of the SLAB allocator.
/// It tracks small objects inside a 1024 Byte page.
pub struct Slab<const N: usize> {
    /// Pointer to the next Slab of this size.
    next: Option<Pin<&mut Self>>,

    /// Pointer to the previous Slab of this size.
    prev: Option<Pin<&mut Self>>,

    /// Object tracking with a granularity of the directed size.
    objs: [u32; {(1024 / N) / 32}],
}


impl<const N: usize> Slab<N> {
    /// Attempts to allocate a new object of `n` number of blocks.
    pub fn allocate(self: Pin<&mut Self>, n: NBlocks) -> Option<*mut u8> {
        // Early bail if this slab is full.
        if self.objs == [u32; {(1024 / N) / 32}] {
            return self.nextslab(n);
        }

        // Get the mask.
        let mut mask = n.mask();

        // Check each objs independently.
        for (i, list) in self.objs.iter_mut().enumerate() {
            for j in 0..n.maxoff() {
                match list & mask {
                    0 => {
                        // Allocate the memory.
                        list |= mask;

                        // Calculate the address.
                        let addr = self.addr + ( i * N * 32 ) + (j * N);

                        return Some( addr as *mut u8 );
                    },

                    _ => (),
                }

                mask << 1;
            }
        }

        // Check if there is a next slab. If so, check if it has space.
        self.nextslab(n)
    }

    /// Request allocation from the next slab or fail if there are None.
    fn nextslab(self: Pin<&mut Self>, n: NBlocks) -> Option<*mut u8> {
        match self.next {
            Some(slab) => slab.allocate(n),
            _ => None,
        }
    }

    /// Autoremove if the slab is empty.
    /// Returns the address of its Slab.
    pub fn autoremove(self: Pin<&mut Self>) -> Option<u32> {
        if self.objs == [0u32; {(1024 / N) / 32}] {
            // Check if there is a previous one.
            // If not, keep, this is the first slab.
            match self.prev {
                Some(p) => {
                    p.next = self.next;
                    return Some( self.addr() )
                },
                None => (),
            }
        }

        None
    }

    /// The address of this Slab's page.
    #[inline(always)]
    fn addr(self: Pin<&mut Self>) -> u32 {
        self.deref() as *const Self as u32
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NBlocks {
    Blocks1,
    Blocks2,
    Blocks3,
    Blocks4,
}

impl NBlocks {
    /// Create the mask.
    #[inline(always)]
    pub fn mask(&self) -> usize {
        match *self {
            NBlocks::Blocks1 => 0b0001,
            NBlocks::Blocks2 => 0b0011,
            NBlocks::Blocks3 => 0b0111,
            NBlocks::Blocks4 => 0b1111,
        }
    }

    /// Create the maximum offset.
    #[inline(always)]
    pub fn maxoff(&self) -> usize {
        match *self {
            NBlocks::Blocks1 => 31,
            NBlocks::Blocks2 => 30,
            NBlocks::Blocks3 => 29,
            NBlocks::Blocks4 => 28,
        }
    }
}