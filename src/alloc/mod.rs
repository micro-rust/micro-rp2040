//! Allocation module.
//! Implements all the functionality of the alloc core module.



/// Index of all the markers. Size of index: 256 bytes.
#[link_section = ".systembss.ALLOCIDX"]
#[used]
static mut ALLOCIDX: [u32; 64] = [0u32; 64];

#[repr(C)]
pub struct Allocator {
    /// Index of all the markers. Size of index: 256 bytes.
    index: [u32; 64],

    /// Markers for all the SRAM region.
    markers: [u32; 1024],
}


impl GlobalAlloc for Allocator {
    #[inline(always)]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        Self::allocate(layout)
    }

    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        Self::deallocate(ptr, layout)
    }

    #[inline(always)]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        Self::allocate_zeroed(layout)
    }

    #[inline(always)]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        Self::reallocate(ptr, layout, new_size)
    }
}