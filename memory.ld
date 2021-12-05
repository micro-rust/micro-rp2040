


EXTERN(Reset0);
ENTRY(Reset0);

/* Provide dummy symbols for main functions */
PROVIDE(__MAINFN0 = 0x00000000);
PROVIDE(__MAINFN1 = 0x00000000);

/* Compile time configuration of IRQs. */
EXTERN(DefaultHandlerFn);


PROVIDE(NMI0 = DefaultHandlerFn);
PROVIDE(NMI1 = DefaultHandlerFn);

PROVIDE(HardFault0 = DefaultHandlerFn);
PROVIDE(HardFault1 = DefaultHandlerFn);

PROVIDE(SVCall0 = DefaultHandlerFn);
PROVIDE(SVCall1 = DefaultHandlerFn);

PROVIDE(PendSV0 = DefaultHandlerFn);
PROVIDE(PendSV1 = DefaultHandlerFn);

PROVIDE(Systick0 = DefaultSystickFn);
PROVIDE(Systick1 = DefaultSystickFn);



/* Compile time configuration of pins. */
PROVIDE(__GPIO00__  = 0);
PROVIDE(__GPIO01__  = 0);
PROVIDE(__GPIO02__  = 0);
PROVIDE(__GPIO03__  = 0);
PROVIDE(__GPIO04__  = 0);
PROVIDE(__GPIO05__  = 0);
PROVIDE(__GPIO06__  = 0);
PROVIDE(__GPIO07__  = 0);
PROVIDE(__GPIO08__  = 0);
PROVIDE(__GPIO09__  = 0);
PROVIDE(__GPIO10__ = 0);
PROVIDE(__GPIO11__ = 0);
PROVIDE(__GPIO12__ = 0);
PROVIDE(__GPIO13__ = 0);
PROVIDE(__GPIO14__ = 0);
PROVIDE(__GPIO15__ = 0);
PROVIDE(__GPIO16__ = 0);
PROVIDE(__GPIO17__ = 0);
PROVIDE(__GPIO18__ = 0);
PROVIDE(__GPIO19__ = 0);
PROVIDE(__GPIO20__ = 0);
PROVIDE(__GPIO21__ = 0);
PROVIDE(__GPIO22__ = 0);
PROVIDE(__GPIO23__ = 0);
PROVIDE(__GPIO24__ = 0);
PROVIDE(__GPIO25__ = 0);
PROVIDE(__GPIO26__ = 0);
PROVIDE(__GPIO27__ = 0);
PROVIDE(__GPIO28__ = 0);
PROVIDE(__GPIO29__ = 0);



PINLOCK =   __GPIO00__ | __GPIO01__ | __GPIO02__ | __GPIO03__ | __GPIO04__ |
            __GPIO05__ | __GPIO06__ | __GPIO07__ | __GPIO08__ | __GPIO09__ |
            __GPIO10__ | __GPIO11__ | __GPIO12__ | __GPIO13__ | __GPIO14__ |
            __GPIO15__ | __GPIO16__ | __GPIO17__ | __GPIO18__ | __GPIO19__ |
            __GPIO20__ | __GPIO21__ | __GPIO22__ | __GPIO23__ | __GPIO24__ |
            __GPIO25__ | __GPIO26__ | __GPIO27__ | __GPIO28__ | __GPIO29__;


/* Memory map of he RP2040 */
MEMORY {
    /* Chip ROM FLASH. Read and execute. */
    ROM(RX ) : ORIGIN = 0x00000000, LENGTH = 16K



    /* 16MB FLASH memory access through XIP cache. Check cache, update on miss. */
    FLASH   (RX ) : ORIGIN = 0x10000000, LENGTH = 16M

    /* 16MB FLASH memory access through XIP cache. Check cache, do not update on miss. */
    FLASHCN (RX ) : ORIGIN = 0x11000000, LENGTH = 16M

    /* 16MB FLASH memory access through XIP cache. Update cache, read after. */
    FLASHNU (RX ) : ORIGIN = 0x12000000, LENGTH = 16M

    /* 16MB FLASH memory access through XIP cache. Bypass cache. */
    FLASHNN (RX ) : ORIGIN = 0x13000000, LENGTH = 16M


    /* 16kB XIP Cache. Reclaimable from the peripheral. */
    XIPRAM (RWX) : ORIGIN = 0x15000000, LENGTH = 16K


    /* 4 x 64kB word stripped SRAM banks. */
    SRAM (RWX) : ORIGIN = 0x20000000, LENGTH = 256K



    /* Core 0 Stack, Vector and System Data section */
    EVECT0  (RWX) : ORIGIN = 0x20040000, LENGTH =  192
    STACK0  (RWX) : ORIGIN = 0x200400C0, LENGTH = 3136
    SYSMEM0 (RWX) : ORIGIN = 0x20040D00, LENGTH =  768

    /* Core 0 Stack, Vector and System Data section */
    EVECT1  (RWX) : ORIGIN = 0x20041000, LENGTH =  192
    STACK1  (RWX) : ORIGIN = 0x200410C0, LENGTH = 3136
    SYSMEM1 (RWX) : ORIGIN = 0x20041D00, LENGTH =  768



    /* 4 x 64kB individual SRAM banks. */
    SRAM0 (RWX) : ORIGIN = 0x21000000, LENGTH = 64K
    SRAM1 (RWX) : ORIGIN = 0x21010000, LENGTH = 64K
    SRAM2 (RWX) : ORIGIN = 0x21020000, LENGTH = 64K
    SRAM3 (RWX) : ORIGIN = 0x21030000, LENGTH = 64K



    /* 4kB USB DPRAM. Reclaimable from the peripheral. */
    USBRAM (RWX) : ORIGIN = 0x50100000, LENGTH = 4K



    /* 2 x 128B PIO Program Data. */
    PIO0PD (RW ) : ORIGIN = 0x50200048, LENGTH = 128
    PIO1PD (RW ) : ORIGIN = 0x50300048, LENGTH = 128
}



SECTIONS {
    /* ******************************************************************** */
    .bootloader ORIGIN(FLASH) : 
    {
        KEEP(*(.boot2));
        KEEP(*(.boot2.*));
    } > FLASH

    . = ORIGIN(FLASH) + 256;



    /* ******************************************************************** */
    .vector_table ORIGIN(FLASH) + 0x100 : 
    {
        /* Core 0 vector table. */
        __svt0 = .;

        LONG( ORIGIN(SYSMEM0) - 4 );
        KEEP(*(.vectortable.Reset0));
        KEEP(*(.vectortable.int0));

        __evt0 = .;



        /* Core 1 vector table. */
        __svt1 = .;

        LONG( ORIGIN(SYSMEM1) - 4 );
        KEEP(*(.vectortable.Reset1));
        KEEP(*(.vectortable.int1));

        __evt1 = .;
    } > FLASH



    /* ******************************************************************** */
    .text :
    {
        . = ALIGN(4);

        /* Reset code. */
        KEEP(*(.Reset.*));

        /* User code. */
        *(.text .text.*);

        /* Hard Fault code. */
        KEEP(*(.HardFault .HardFault.*));

        /* Default handlers */
        KEEP(*(.default .default.*));

        . = ALIGN(4);
    } > FLASH
    . = ALIGN(4);



    /* ******************************************************************** */
    .rodata : ALIGN(4)
    {
        *(.rodata .rodata.*);

        LONG(__MAINFN0);
        LONG(__MAINFN1);

        . = ALIGN(4);
    } > FLASH



    /* DATA section. */
    /* ******************************************************************** */
    .data : ALIGN(4)
    {
        . = ALIGN(4);

        __sdata = .;

        *(.data .data.*);

        . = ALIGN(4);
    } > SRAM AT >FLASH

    . = ALIGN(4);

    __edata = .;
    __ldata = LOADADDR(.data);



    /* BSS section. */
    /* ******************************************************************** */
    . = ALIGN(4);
    __sbss = .;

    .bss (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);

        *(.bss .bss.*);
        *(COMMON);

        . = ALIGN(4);
    } > SRAM

    . = ALIGN(4);
    __ebss = .;



    /* UNINIT section. */
    /* ******************************************************************** */
    . = ALIGN(4);
    __suninit = .;

    .uninit : ALIGN(4)
    {
        . = ALIGN(4);

        *(.uninit .uninit.*);

        . = ALIGN(4);
    } > SRAM AT >FLASH

    . = ALIGN(4);
    __euninit = .;
    __luninit = LOADADDR(.uninit);




    /* System DATA and BSS section. */
    /* ******************************************************************** */
    .sysdata0 : ALIGN(4)
    {
        . = ALIGN(4);

        __ssysdata0 = .;

        *(.systemdata0 .systemdata0.*);

        LONG(PINLOCK);

        . = ALIGN(4);
    } > SYSMEM0 AT >FLASH

    . = ALIGN(4);

    __esysdata0 = .;
    __lsysdata0 = LOADADDR(.sysdata0);


    . = ALIGN(4);
    __ssysbss0 = .;

    .sysbss0 (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);

        *(.systembss0 .systembss0.*);

        . = ALIGN(4);
    } > SYSMEM0

    . = ALIGN(4);
    __esysbss0 = .;
    /* ******************************************************************** */
    .sysdata1 : ALIGN(4)
    {
        . = ALIGN(4);

        __ssysdata1 = .;

        *(.systemdata1 .systemdata1.*);

        LONG(PINLOCK);

        . = ALIGN(4);
    } > SYSMEM1 AT >FLASH

    . = ALIGN(4);

    __esysdata1 = .;
    __lsysdata1 = LOADADDR(.sysdata1);


    . = ALIGN(4);
    __ssysbss1 = .;

    .sysbss1 (NOLOAD) : ALIGN(4)
    {
        . = ALIGN(4);

        *(.systembss1 .systembss1.*);

        . = ALIGN(4);
    } > SYSMEM1

    . = ALIGN(4);
    __esysbss1 = .;
    /* ******************************************************************** */


    /* Discarded sections. */
    /DISCARD/ :
    {
        *(.ARM.exidx);
        *(.ARM.exidx.*);
        *(.ARM.extab.*);
    }
}
