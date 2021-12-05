//! Initialization procedure of the Memory Protection Unit.
//! Implements stack guards, as well as protection for the other Core's stack.

pub(crate) fn mpu() {
    crate::sys::init::mpu(0x20040000, 0x20041000)
}
