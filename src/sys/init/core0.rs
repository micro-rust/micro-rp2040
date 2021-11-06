//! Core 0 Reset function.
//! Initializes and configures the basic peripherals for the system.
//! Configures and allows the use of the pseudo-kernel's functions.


#[link_section = ".vectortable.Reset0"]
#[no_mangle]
#[used]
static RESET0 : fn() -> ! = Reset0;



pub(crate) fn Reset0() -> ! {
    micro::asm::bkpt::<5>();

    // Load RAM sections.
    unsafe { sections(); }

    // Initialize the clocks.
    initialize();

    // Start core 1.
    wakeup(super::core1::Reset1);

    // Jump to user code.
    jump0()
}



/// Loads the RAM sections and zeros the .bss and .uninit.
unsafe fn sections() {
    use super::load;
    use super::zero;

    extern "C" {
        static mut __sdata : u32;
        static mut __edata : u32;
        static __ldata : u32;
    }

    load(
        &mut __sdata as *mut u32,
        &mut __edata as *mut u32,
        & __ldata as *const u32,
    );

    extern "C" {
        static mut __ssysdata : u32;
        static mut __esysdata : u32;
        static __lsysdata : u32;
    }

    load(
        &mut __ssysdata as *mut u32,
        &mut __esysdata as *mut u32,
        & __lsysdata as *const u32,
    );

    extern "C" {
        static mut __sbss : u32;
        static mut __ebss : u32;
    }

    zero(
        &mut __sbss as *mut u32,
        &mut __ebss as *mut u32,
    );

    extern "C" {
        static mut __suninit : u32;
        static mut __euninit : u32;
    }

    zero(
        &mut __suninit as *mut u32,
        &mut __euninit as *mut u32,
    );

    extern "C" {
        static mut __ssysbss : u32;
        static mut __esysbss : u32;
    }

    zero(
        &mut __ssysbss as *mut u32,
        &mut __esysbss as *mut u32,
    );
}


/// Initialize the RP2040.
fn initialize() {
    // Initialization routine of the Reset peripheral.
    crate::sys::power::RESET.init();

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


#[allow(warnings)]
#[inline(never)]
/// Starts Core 1.
fn wakeup(entry: fn() -> !) {
    use crate::raw::SIORegister;
    use crate::sync::Mailbox;

    use micro::Register;

    extern "C" {
        fn multicore_trampoline();
    }

    // Reset in the PSM.
    let psm = unsafe { &mut *(0x40010004 as *mut [SIORegister<u32>; 4]) };

    psm[1].set(1 << 16);

    while (psm[1].read() >> 16) & 1 == 0 { micro::asm::nop() }

    psm[1].clear(1 << 16);

    // Get vector table.
    let vectortable = 0x10000140;

    // Get Core 1 stack pointer.
    let corestack = (0x20040000 as *mut [u8; 2048]) as *mut usize;
    let mut sp = unsafe { corestack.add( 2048 / core::mem::size_of::<usize>() ) };

    // Write initial data.
    sp = unsafe { sp.sub(3) };

    unsafe {
        sp.add(0).write_volatile(entry as usize);
        sp.add(1).write_volatile(corestack as usize);
        sp.add(2).write_volatile(wrapper as usize);
    }

    // Build command sequence.
    let sequence = [0, 0, 1, vectortable, sp as u32, multicore_trampoline as u32];

    // Start sending commands.
    let mut i = 0;
    let mut last = 0;
    let mut maxi = 0;

    let mut responses = [0u32; 32];

    micro::asm::bkpt::<1>();

    loop {
        let cmd = sequence[i];

        if cmd == 0 {
            Mailbox::drain();
            micro::asm::sev();
        }

        Mailbox::send_blocking(cmd);

        let response = Mailbox::recv_blocking();

        i = if cmd == response { i + 1 } else {
            if last < 32 { responses[last] = response; last += 1; }
            if last >= 6 { micro::asm::bkpt::<5>() }
            0
        };

        maxi = i;

        if i >= sequence.len() {
            break;
        }
    }

    micro::asm::bkpt::<2>();
}

#[allow(improper_ctypes_definitions)]
extern "C" fn wrapper(entry: fn() -> !, _: *mut ()) -> ! {
    // Setup.
    // Go to entry.
    entry()
}


/// Jumps to user code.
/// If there is no user code, hangs in a debug loop.
fn jump0() -> ! {
    extern "C" {
        static __MAINFN0 : extern fn() -> !;
    }

    micro::asm::bkpt::<137>();

    match unsafe { __MAINFN0 } as u32 {
        0x00000000 => loop { micro::asm::bkpt::<138>(); },
        _ => unsafe { __MAINFN0() },
    }
}
