//! `micro-rp2040` dma example.

#![no_std]
#![no_main]

use micro_rp2040 as rp2040;

use rp2040::{ main0 };


main0!(usermain);

static mut COUNT : u32 = 0u32;


fn usermain() -> ! {
    use rp2040::peripherals::dma::{
        DMAChannel, DMAHandle, Stream,
        buffer::{ CopyFromRam, CopyIntoRam, SourceBuffer, DestinationBuffer }
    };


    // Create the buffers.
    let data = unsafe { &mut *(0x21000000 as *mut [u32; 256]) };
    let sink = unsafe { &mut *(0x21000400 as *mut [u32; 256]) };

    // Populate the source buffer.
    for i in 0..256 {
        data[i as usize] = i;
    }

    // Validate the buffers.
    let data = CopyFromRam::create(data).unwrap();

    let sink = CopyIntoRam::create(sink).unwrap();

    // Acquire the DMA Channel.
    let mut channel = DMAChannel::<0>::acquire().unwrap();

    // Test the stream.
    // Options: Do not byte swap, low priority.
    let mut stream = Stream::copy(&mut channel, data, sink, None).unwrap();

    // Get the handle to the stream.
    let handle = stream.launch();

    // You can do stuff here.
    // The stream will execute on its own.
    // I, for example, am calculating what's 2 + 2, cause my calculator broke.
    let x = 2;
    let y = 2;

    let z = x + y;

    // Now that we have done our things, wait until the stream completes.
    // Doing some tests, the DELTA for same bank transfers takes ~17 microseconds.
    handle.join();

    loop { nop() }
}
