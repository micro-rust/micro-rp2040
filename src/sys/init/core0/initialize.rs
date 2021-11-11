//! Initializes the RP2040 bare minimum peripherals.

pub(super) fn initialize() {
    // Initialization routine of the Reset peripheral.
    crate::sys::power::RESET.init();

    // Load ROM Function pointers and data.
    unsafe { romfunc(); }

    micro::asm::bkpt::<0>();

    // Initialization routine of the Clocks peripheral.
    unsafe { crate::sys::CLOCKS.init(); }

    // Initialize the remaining peripherals.
    crate::sys::power::RESET.finish();

    // Initialize interrupts.
    crate::sys::ints::InterruptSystem::init();

    for i in 0..16 {
        unsafe { crate::sys::TESTCLOCKS[i] = crate::sys::CLOCKS.freqs[i] }
    }
}


/// Loads all the ROM function pointers.
#[allow(mutable_transmutes)]
#[inline(never)]
unsafe fn romfunc() {
    use crate::math::*;

    use core::ptr::{
        read_volatile as read,
        write_volatile as write,
    };


    // Load the Soft Float table pointer.
    let sf: u32 = rom_table_lookup(0x16 as *const u16, *b"SF");

    // Get size of bootrom table.
    let size = SFTABLE.0.len();

    // Destination pointer.
    let mut dest: *mut u32 = core::mem::transmute( &SFTABLE as *const _ );

    // Source pointer.
    let mut source: *const u16 = sf as *const _;

    micro::asm::bkpt::<1>();

    for _ in 0..size {
        let pointer = read(source) as u32;

        write(dest, pointer);
        dest = dest.offset(1);
        source = source.offset(2);
    }

    //write((&crate::math::SFTABLE) as *const usize as *mut usize, sf)
}

type RomTableLookupFn<T> = unsafe extern "C" fn(*const u16, u32) -> T;

fn rom_table_lookup<T>(table: *const u16, tag: [u8; 2]) -> T {
    const PTR: *const u16 = 0x18 as _;

    unsafe {
        let rom_table_lookup_ptr: *const u32 = rom_hword_as_ptr(PTR);
        let rom_table_lookup: RomTableLookupFn<T> = core::mem::transmute(rom_table_lookup_ptr);

        rom_table_lookup(
            rom_hword_as_ptr(table) as *const u16,
            u16::from_le_bytes(tag) as u32,
        )
    }
}

unsafe fn rom_hword_as_ptr(addr: *const u16) -> *const u32 {
    let ptr: u16 = *addr;
    ptr as *const u32
}