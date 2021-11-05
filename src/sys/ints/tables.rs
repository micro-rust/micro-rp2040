//! IRQ and Exception tables.


pub(super) type EXCTable = micro::int::EXCTable;
pub(super) type IRQTable = micro::int::IRQTable<32>;


extern "C" {
    fn NMI0();
    fn HardFault0();
    fn SVCall0();
    fn PendSV0();
    fn Systick0();

    fn NMI1();
    fn HardFault1();
    fn SVCall1();
    fn PendSV1();
    fn Systick1();
}


pub union Vector {
    f: unsafe extern "C" fn(),
    r: u32,
}



#[link_section = ".vectortable.int0"]
#[used]
pub(crate) static VTABLE0 : [Vector; 14] = [
    Vector { f: NMI0 },
    Vector { f: HardFault0 },

    Vector { r: 0u32 },
    Vector { r: 0u32 },
    Vector { r: 0u32 },
    Vector { r: 0u32 },
    Vector { r: 0u32 },
    Vector { r: 0u32 },
    Vector { r: 0u32 },

    Vector { f: SVCall0 },

    Vector { r: 0u32 },
    Vector { r: 0u32 },

    Vector { f: PendSV0 },
    Vector { f: Systick0 },
];


#[link_section = ".vectortable.int1"]
#[used]
pub(crate) static VTABLE1 : [Vector; 14] = [
    Vector { f: NMI1 },
    Vector { f: HardFault1 },

    Vector { r: 0u32 },
    Vector { r: 0u32 },
    Vector { r: 0u32 },
    Vector { r: 0u32 },
    Vector { r: 0u32 },
    Vector { r: 0u32 },
    Vector { r: 0u32 },

    Vector { f: SVCall1 },

    Vector { r: 0u32 },
    Vector { r: 0u32 },

    Vector { f: PendSV1 },
    Vector { f: Systick1 },
];
