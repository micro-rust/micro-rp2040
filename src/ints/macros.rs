//! Interruot macros.


#[macro_export]
macro_rules! nmi {
    (Core0, $fn:ident) => {
        #[no_mangle]
        pub static __NMI0__ : unsafe extern "C" fn() = $fn;
    };

    (Core1, $fn:ident) => {
        #[no_mangle]
        pub static __NMI1__ : unsafe extern "C" fn() = $fn;
    };

    ($fn:ident) => {
        #[no_mangle]
        pub static __NMI0__ : unsafe extern "C" fn() = $fn;

        #[no_mangle]
        pub static __NMI1__ : unsafe extern "C" fn() = $fn;
    };
}


#[macro_export]
macro_rules! hardfault {
    (Core0, $fn:ident) => {
        #[no_mangle]
        #[used]
        pub static __HARDFAULT0__ : unsafe extern "C" fn() = $fn;
    };

    (Core1, $fn:ident) => {
        #[no_mangle]
        #[used]
        pub static __HARDFAULT1__ : unsafe extern "C" fn() = $fn;
    };

    ($fn:ident) => {
        #[no_mangle]
        #[used]
        pub static __HARDFAULT0__ : unsafe extern "C" fn() = $fn;

        #[no_mangle]
        #[used]
        pub static __HARDFAULT1__ : unsafe extern "C" fn() = $fn;
    };
}


#[macro_export]
macro_rules! svcall {
    (Core0, $fn:ident) => {
        #[no_mangle]
        #[used]
        pub static __SVCALL0__ : unsafe extern "C" fn() = $fn;
    };

    (Core1, $fn:ident) => {
        #[no_mangle]
        #[used]
        pub static __SVCALL1__ : unsafe extern "C" fn() = $fn;
    };

    ($fn:ident) => {
        #[no_mangle]
        #[used]
        pub static __SVCALL0__ : unsafe extern "C" fn() = $fn;

        #[no_mangle]
        #[used]
        pub static __SVCALL1__ : unsafe extern "C" fn() = $fn;
    };
}


#[macro_export]
macro_rules! pendsv {
    (Core0, $fn:ident) => {
        #[no_mangle]
        #[used]
        pub static __PENDSV0__ : unsafe extern "C" fn() = $fn;
    };

    (Core1, $fn:ident) => {
        #[no_mangle]
        #[used]
        pub static __PENDSV1__ : unsafe extern "C" fn() = $fn;
    };

    ($fn:ident) => {
        #[no_mangle]
        #[used]
        pub static __PENDSV0__ : unsafe extern "C" fn() = $fn;

        #[no_mangle]
        #[used]
        pub static __PENDSV1__ : unsafe extern "C" fn() = $fn;
    };
}


#[macro_export]
macro_rules! systick {
    (Core0, $fn:ident) => {
        #[no_mangle]
        pub static __SYSTICK0__ : unsafe extern "C" fn() = $fn;
    };

    (Core1, $fn:ident) => {
        #[no_mangle]
        pub static __SYSTICK1__ : unsafe extern "C" fn() = $fn;
    };

    ($fn:ident) => {
        #[no_mangle]
        pub static __SYSTICK0__ : unsafe extern "C" fn() = $fn;

        #[no_mangle]
        pub static __SYSTICK1__ : unsafe extern "C" fn() = $fn;
    };
}
