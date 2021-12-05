//! Allocation module.
//! Implements all the functionality of the alloc core module.


#![deny(warnings)]


mod page;


use core::alloc::GlobalAlloc;
use core::alloc::Layout;

use crate::sync::AllocatorLock;

use self::page::Page;


type BigPage = Page<128>;
type SmallPage = Page<4>;



#[global_allocator]
#[link_section = ".systembss.ALLOCATOR"]
static ALLOCATOR: AllocatorWrapper = AllocatorWrapper;


/// Wrapper for the allocator.
pub struct AllocatorWrapper;


unsafe impl GlobalAlloc for AllocatorWrapper {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Wait for the allocator lock.
        loop {
            match AllocatorLock::acquire() {
                Some(_) => {
                    // Create reference to the real allocator.
                    let allocator = &mut *(0x20000000 as *mut MicroAllocator);

                    match allocator.allocate(layout) {
                        Some(addr) => return addr,
                        _ => return 0x00000000 as *mut u8,
                    }
                },
                _ => continue,
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Wait for the allocator lock.
        loop {
            match AllocatorLock::acquire() {
                Some(_) => {
                    // Create reference to the real allocator.
                    let allocator = &mut *(0x20000000 as *mut MicroAllocator);

                    allocator.deallocate(ptr, layout);
                },
                _ => continue,
            }
        }
    }
}


/// Total size = 64 * 8 bytes + 64 * 4 bytes = 768 bytes = 6 small pages.
pub struct MicroAllocator<'a> {
    /// Big pages to perform big allocations of objects and smaller pages.
    /// 64 pages * 4 kB / page = 256 kB.
    big: [BigPage; 64],

    /// Small pages to perform allocations of small objects.
    /// 64 arrays * 32 pages / array * 128 bytes / page = 256 kB.
    small: [Option<&'a mut [SmallPage; 32]>; 64],
}


impl<'a> MicroAllocator<'a> {
    /// Creates the Allocator and initializes it.
    pub(crate) fn init() {
        // Build at stripped memory.
        let base = 0x20000000;

        // Reference to the allocator.
        let allocator = unsafe { &mut *(base as *mut Self) };

        // Initialize all big pages.
        BigPage::array( base as *mut _, base);
        BigPage::array( (base + 256) as *mut _, base+ (4096 * 32) );

        // Initialize all small pages pointers.
        let small = unsafe { &mut *((base + 512) as *mut [Option<&'a mut [SmallPage; 32]>; 64]) };

        for item in small.iter_mut() {
            *item = None;
        }

        // Reserve first 8 pages.
        allocator.big[0].region0();
    }

    /// Allocates a new object.
    pub(self) unsafe fn allocate(&mut self, layout: Layout) -> Option<*mut u8> {
        match layout.size() {
            0..=96 => self.allocsmall(layout),
            _ => self.allocbig(layout),
        }
    }

    /// Deallocates an object.
    pub(self) unsafe fn deallocate(&mut self, ptr: *mut u8, layout: Layout) {
        // Check size.
        match layout.size() {
            0..=96 => self.deallocsmall(ptr, layout),
            _ => self.deallocbig(ptr, layout),
        }
    }


    /// Allocates a big object (up to 4096 for now) (maximum alignment 128 bytes).
    unsafe fn allocbig(&mut self, layout: Layout) -> Option<*mut u8> {
        for page in self.big.iter_mut() {
            if let Some(addr) = page.alloc::<128>(layout.size() as u32) {
                return Some(addr);
            }
        }

        None
    }

    /// Deallocates a big object (up to 4096 for now) (maximum alignment 128 bytes).
    unsafe fn deallocbig(&mut self, ptr: *mut u8, layout: Layout) {
        for page in self.big.iter_mut() {
            if page.contains(ptr) {
                page.dealloc(ptr, layout.size() as u32);
                return;
            }
        }
    }

    /// Allocates a small object.
    unsafe fn allocsmall(&mut self, layout: Layout) -> Option<*mut u8> {
        // Indicates if there are free small pages.
        let mut free = None;

        // Check all small pages for free space.
        for (i, item) in self.small.iter_mut().enumerate() {
            if let Some(array) = item {
                for page in array.iter_mut() {
                    match layout.align() {
                        0..=4 => if let Some(addr) = page.alloc::<04>(layout.size() as u32) { return Some( addr ) },
                        8     => if let Some(addr) = page.alloc::<08>(layout.size() as u32) { return Some( addr ) },
                        16    => if let Some(addr) = page.alloc::<16>(layout.size() as u32) { return Some( addr ) },
                        32    => if let Some(addr) = page.alloc::<32>(layout.size() as u32) { return Some( addr ) },
                        64    => if let Some(addr) = page.alloc::<64>(layout.size() as u32) { return Some( addr ) },

                        _ => unimplemented!(),
                    }
                }
            } else {
                free = Some(i);
            }
        }

        // If no small pages were free, either the device is OOM or memory is too fragmented.
        let free = match free {
            Some(f) => f,
            _ => return None,
        };

        // If this failed, reserve a new Big Page.
        let mut region = None;

        for page in &mut self.big {
            if page.empty() {
                page.reserve();

                region = Some(page.addr());

                break;
            }
        }

        // A new 4096 byte page was reserved, look for a new 128 byte space to place the page controls.
        let mut control = None;

        for page in self.big.iter_mut() {
            if let Some(addr) = page.alloc::<128>(256) {
                control = Some(addr);

                break;
            }
        }

        // We have a place for a new array and a new big page.
        if let (Some(region), Some(control)) = (region, control) {
            // Create a new Small Page array.
            let array = SmallPage::array(control, region);

            let addr = match layout.align() {
                0..=4 => if let Some(addr) = array[0].alloc::<04>(layout.size() as u32) { addr } else { unreachable!() },
                8     => if let Some(addr) = array[0].alloc::<08>(layout.size() as u32) { addr } else { unreachable!() },
                16    => if let Some(addr) = array[0].alloc::<16>(layout.size() as u32) { addr } else { unreachable!() },
                32    => if let Some(addr) = array[0].alloc::<32>(layout.size() as u32) { addr } else { unreachable!() },
                64    => if let Some(addr) = array[0].alloc::<64>(layout.size() as u32) { addr } else { unreachable!() },

                _ => unimplemented!(),
            };

            self.small[free] = Some(array);

            return Some(addr);
        }

        None
    }

    /// Deallocates a small object.
    unsafe fn deallocsmall(&mut self, ptr: *mut u8, layout: Layout) {
        // Check all small pages for free space.
        for item in self.small.iter_mut() {
            if let Some(array) = item {
                for page in array.iter_mut() {
                    if page.contains(ptr) {
                        page.dealloc(ptr, layout.size() as u32);

                        return;
                    }
                }
            }
        }
    }
}