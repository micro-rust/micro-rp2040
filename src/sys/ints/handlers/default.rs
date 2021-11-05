//! Default handlers.


#[no_mangle]
fn DefaultSystickFn() {
	return;
}

#[no_mangle]
fn DefaultHandlerFn() {
	micro::asm::bkpt::<255>();
}