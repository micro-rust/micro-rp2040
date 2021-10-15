//! Interrupt module.


#[macro_use]
mod macros;

pub(crate) mod vectortable;



#[no_mangle]
#[inline(never)]
fn DefaultHandlerFn() {
    loop { micro::asm::bkpt::<255>() }
}


#[no_mangle]
#[inline(never)]
fn DefaultSystickFn() {}

