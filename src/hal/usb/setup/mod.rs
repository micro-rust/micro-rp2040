//! Setup module.
//! Handles the setup of the USB bus.






/// Handle setting up the device.
pub fn device_setup(packet: &SetupPacket) {
    match RequestType::from(packet.bmRequestType) {
        RequestType::Standard => match RequestDirection::from(packet.bmRequestType) {
            RequestDirection::In => {

                match Request::from(packet.bRequest) {
                    Request::GetStatus => {
                        // Prepare to send 2 bytes.
                    },

                    Request::GetDescriptor => {
                        // Create a new transfer to send the USB descriptor requested.
                        // 
                    },

                    Request::GetConfiguration => {
                        // Swicth to the given USB Configuration.
                    },

                    // Request sent in wrong direction??
                    _ => {}
                }

                if let Some(len) = len {
                    // in_packet.data_len = MIN(len, packet.wLength);
                    // Start the Single Buffer Control In transfer.
                }
            },

            RequestDirection::Out => match Request::from(packet.bRequest) {
                // Ignore this, no feature to set.
                Request::SetFeature => {},

                // Set the address of the USB Device.
                Request::SetAddress => {
                    // Get the address.
                    let addr = packet.wValue;

                    // Validate address.
                    if (addr > 0) && (addr < 128) {
                        // Set the address.
                        // usb_start_empty_control_in_transfer
                    }
                },

                // Ignore this, no descriptor configuration yet.
                Request::SetDescriptor => {},

                // Set the requested configuration.
                Request::SetConfiguration => {
                    // Get configuration number.
                    let num = packet.wValue;

                    // Validate configuration number.
                    if (num == 0) /* || CHECK HERE IF THE CONFIG EXISTS */ {
                        // usb_handle_set_config
                        // usb_start_empty_control_in_transfer_null_completion
                    }
                },
            },
        },

        _ => (),
    }

    // Send a stall.
    // usb_stall_control_pipe
}



/// Handle setting up an interface.
pub fn interface_setup(packet: &SetupPacket /*, interface: &Interface*/) {

    match RequestType::from(packet.bmRequestType) {

        // Respond only to Standard requests.
        RequestType::Standard => match RequestDirection::from(packet.bmRequestType) {

            // Handle the Device -> Host transfer.
            RequestDirection::In => match Request::from(packet.bRequest) {

                // Send the status to the Host.
                Request::GetStatus => {
                    // usb_start_tiny_control_in_transfer
                },

                // Send the interface to the Host.
                Request::GetInterface => {
                    if (packet.wValue > 0) && (packet.wLength == 1) {
                        // usb_start_tiny_control_in_transfer(interface.alt, 1)
                    }

                    // Go to stall.
                },

                // Request does not belong here??
                _ => (),
            },


            // Handle the Host -> Device transfer.
            RequestDirection::Out => match RequestDirection::from(packet.bRequest) {
                // Set the interface given.
                Request::SetInterface => {
                    // Check if the interface has an alternate.
                    if interface.alternate() {
                        if interface.set_alternate_handler(interface, packet.wValue) {
                            interface.alt = packet.wValue;
                            // usb_start_empty_control_in_transfer_null_complete
                        }
                    }
                },

                // Request does not belong here??
                _ => (),
            },
        },

        _ => (),
    }

    // Send a stall.
    // usb_stall_control_pipe
}


/// Handle setting up an endpoint.
pub fn endpoint_setup(packet: &SetupPacket /*, ep: &Endpoint*/) {

    match RequestType::from(packet.bmRequestType) {

        // Respond only to standard requests.
        RequestType::Standard => match RequestDirection::from(packer.bmRequestType) {

            // Handle setup from Device to Host.
            RequestDirection::In => match Request::from(packet.bRequest) {

                // Send status to Host.
                Request::GetStatus => {
                    if (packet.wValue == 0) && (packet.wLength == 2) {
                        // usb_start_tiny_control_in_transfer(ep.halt_state > HS_NON_HALT_STALL ? 1 : 0, 2)
                    }
                },

                _ => (),
            },

            // Handle setup from Host to Device.
            RequestDirection::out => match Request::from(packet.bRequest) {

                // Clear Feature.
                Request::ClearFeature => {
                    if packet.wValue == USB_FEAT_ENDPOINT_HALT {
                        if ep.halt_state() < HS_HALTED_ON_CONDITION {
                            // usb_hard_reset_endpoint
                        }

                        ep.next_pid = 0;

                        // usb_start_empty_control_in_trasnfer_null_completion
                    }
                },

                // Set Feature.
                Request::SetFeature => {
                    // Halt the endpoint.
                    if packet.wValue == USB_FEAT_ENDPOINT_HALT {
                        // usb_stall_endpoint
                        // usb_start_empty_contol_in_transfer_null_complete
                    }
                },

                _ => (),
            },
        },

        _ => (),
    }

    // Send a stall.
    // usb_stall_control_pipe
}


pub fn handle_setup_packet(packet: &SetupPacket) {
    // Reset USB Control IN
    // Reset USB Control OUT

    // Reset USB Control IN / OUT next PID to 1

    match RequestRecipient::from(packet.bmRequestType) {
        RequestRecipient::Device => return device_setup(packet),

        // Handle the Interface Setup packet.
        RequestRecipient::Interface => match find_interface(packet.wIndex) {
            Some(interface) => return interface_setup(packet, interface),
            None => { /* Returns some error */ },
        },

        // Handle the Endpoint Setup packet.
        RequestRecipient::Endpoint => match find_endpoint(packet.wIndex) {
            Some(endpoint) => return endpoint_setup(packet, endpoint),
            None => { /* Returns some error */ },
        },
    }

    // Stall
    // usb_stall_control_pipe
}


pub fn handle_buffer() {
    // Get buffer status.
    let mut buffers = usb.buf_status; // 0x58 -> 22 (23)
    let mut remaining = buffers;

    // Check if buffers is empty -> Spurious error.
    if buffers == 0 {
        return;
    }

    // Loop over all buffers.
    for i in 0..32 {
        if remaining & (1 << i) {
            let which = match usb.buf_cpu_should_handle & (1 << i) {
                0 => false,
                _ => true,
            };

            usb.buf_status.clear(1 << i);

            usb_handle_transfer(i >> 1, !(i & 1), which);

            remaining &= !(1 << i);
        }
    }

    usb.clear(remaining);
}



pub fn isr() {
    // Get reference to the USB Control registers.
    let mut usbhw = unsafe { &mut *(0x50110000 as *mut [AtomicRegister<u32>; 39]) };

    // Get interrupt status.
    let status = usb.ints.read();

    // Marker for handled interrupts.
    let mut handled = 0;

    // Check for a Connection / Disconnection.
    if (status & (1 << 13)) != 0 {
        handled |= 1 << 13;
        usb_handle_conn();
        usb.sie_status.clear(1 << 13);
    }

    // Check for a VBus detection.
    if (status & (1 << 11)) != 0 {
        handled |= 1 << 11;
        usb_handle_vbus();
        usb.sie_status.clear(1 << 11);
    }

    // Check that the USB is initialized and ready.
    /*
        TODO : Implement checking
    */

    // If the USB is not initialized and ready, initialize it and return from ISR.
    /*
        TODO : Implement initialization.
    */


    // Check for a Setup Packet received.
    if (status & (1 << 16)) != 0 {
        handled |= 1 << 16;
        usb_handle_setup_packet(&SETUPPACKET);
        usb.site_status.clear(1 << 16);
    }

    // Handle incoming buffer.
    if (status & (1 << 4)) != 0 {
        handled |= 1 << 4;
        usb_handle_buffer();
        // interrupt is auto cleared.
    }



    // Handle a Bus Reset interrupt.
    const RESET: u32 = 1 << 12;

    if (status & RESET) != 0 {
        // Set the handled flag.
        handled |= RESET;

        usb_handle_bus_reset();

        // Clear the flag from SIE STATUS.
        usbhw[20].write(1 << 19);

        // WTF is this
        #[cfg(feature = "BUG-RP2040-E5")]
        { rp2040_usb_device_enumeration_fix(); }
    }



    // Check for errors. DATA_SEQ | BIT_STUFF | CRC | RX_OVERFLOW | RX_TIMEOUT
    const ERRORS: u32 = (1 << 9) | (1 << 8) | (1 << 7) | (1 << 6) | (1 << 5);

}

#[cfg(any(feature = "BUG-RP2040-E2", feature = "BUG-RP2040-E3", feature = "BUG-RP2040-E4", feature = "BUG-RP2040-E5"))]
#[doc = "Workarounds for RP2040 USB hardware bugs present in revisions B0 and B1."]
mod bugfix {

    #[cfg(feature = "BUG-RP2040-E5")]
    pub use e5::BUG_RP2040_E5;

    #[cfg(feature = "BUG-RP2040-E5")]
    mod e5 {
        /// Line State SE0.
        const SE0 : u32 = 0b00;
        /// Line State SE1.
        const SE1 : u32 = 0b11;

        /// Line State J.
        const J : u32 = 0b01;
        /// Line State K.
        const K : u32 = 0b10;

        /// Fix for bug RP2040-E5.
        pub(crate) fn BUG_RP2040_E5() {
            // Do a check for revision B0/B1 in case the user set this flag incorrectly.
            if chipversion == 1 {
                // Set a callback in 10-15 ms.
                // Use systick as this is a priviledged routine.
                Systick::callback(1, e5callback0);
            }
        }

        /// Callback for bug RP2040-E5 that waits for SE0 signal end.
        fn e5callback0() -> u32 {
            // Reference to the USB Control registers.
            let mut usbhw = unsafe { &mut *(0x50110000 as *mut [AtomicRegister<u32>; 39]) };

            // Check the hardware line state.
            match (usbhw[20].read() >> 2) & 0b11 {
                // If it's still in SE0 wait for 1 ms.
                0b00 => return CallbackResult::Reload(1),

                // If SE0 already ended, force LS and swap with it's callback.
                _ => {

                },
            }
        }

        /// Callback for bug RP2040-E5 that finishes a Bus Reset.
        fn e5callback1() {
            // Wait until the USB thinks it's connected again.

            // Switch back to USB PHY.
            usbhw[29].write((1 << 3) | 1);

            // Get rid of DP Pull Up override.
            usbhw[32].clear(1 << 2);

            // Restore GPIO configuration.

        }
    }
}


pub enum CallbackResult {
    /// The callback is removed.
    None,

    /// The callback is scheduled in the given ms interval.
    Reload(u32),

    /// The callback is swapped with a different one.
    Swap(u32),
}


/// Handler of Bus Reset interrupts.
fn busreset() {
    // Downgrade to unconfigured state.
    // TODO : Perform changes in RAM model too.

    // Reset all endpoints.


    // Downgrade to unadressed state.
    // TODO : Perform changes in RAM model too.
    usbhw[0].write(0);

    // Clear Buffer Status and SIE Status.
    usbhw[22].write(0xFFFFFFFF);
    usbhw[20].write(0xFFFFFFFF);
}

fn epreset() {

}



#[repr(packed)]
pub struct SetupPacket {
    /// Request type.
    bmRequestType: u8,

    /// Request.
    bRequest: u8,

    /// Value.
    wValue: u16,

    /// Index.
    wIndex: u16,

    /// Length of the packed.
    wLength: u16,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestDirection {
    In,
    Out,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestType {
    /// Standard request.
    Standard = 0x00,

    /// Class request.
    Class = 0x20,

    /// Vendor request.
    Vendor = 0x40,
}
