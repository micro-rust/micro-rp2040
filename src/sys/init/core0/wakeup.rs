//! Wakes up Core 1.

#[allow(warnings)]
#[inline(never)]
/// Starts Core 1.
pub(super) fn wakeup() {
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

    // Get Core 1 entry.
    let entry: fn() -> ! = crate::sys::init::core1::Reset1;

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
}


#[allow(improper_ctypes_definitions)]
extern "C" fn wrapper(entry: fn() -> !, _: *mut ()) -> ! {
    // Setup.
    // Go to entry.
    entry()
}
