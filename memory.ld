


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
	FLASHCU (RX ) : ORIGIN = 0x10000000, LENGTH = 16M

	/* 16MB FLASH memory access through XIP cache. Check cache, do not update on miss. */
	FLASHCN (RX ) : ORIGIN = 0x11000000, LENGTH = 16M

	/* 16MB FLASH memory access through XIP cache. Update cache, read after. */
	FLASHNU (RX ) : ORIGIN = 0x12000000, LENGTH = 16M

	/* 16MB FLASH memory access through XIP cache. Bypass cache. */
	FLASHNN (RX ) : ORIGIN = 0x13000000, LENGTH = 16M


	/* 16kB XIP Cache. Reclaimable from the peripheral. */
	XIPRAM (RWX) : ORIGIN = 0x15000000, LENGTH = 16K


	/* 4 x 64kB word stripped SRAM banks. */
	SRAMWS (RWX) : ORIGIN = 0x20000000, LENGTH = 256K

	/* 2 x 4kB SRAM banks. */
	SRAM4 (RWX) : ORIGIN = 0x20040000, LENGTH = 4K
	SRAM5 (RWX) : ORIGIN = 0x20041000, LENGTH = 4K

	/* 4 x 64kB individual SRAM banks. */
	SRAM0 (RWX) : ORIGIN = 0x21000000, LENGTH = 64K
	SRAM1 (RWX) : ORIGIN = 0x21010000, LENGTH = 64K
	SRAM2 (RWX) : ORIGIN = 0x21020000, LENGTH = 64K
	SRAM3 (RWX) : ORIGIN = 0x21030000, LENGTH = 64K


	/* 4kB USB DPRAM. Reclaimable from the peripheral. */
	USBRAM (RWX) : ORIGIN = 0x50100000, LENGTH = 4K


	/* 2 x 128B PIO Program Data. */
	PIO0PD ( W ) : ORIGIN = 0x50200048, LENGTH = 128
	PIO1PD ( W ) : ORIGIN = 0x50300048, LENGTH = 128
}



SECTIONS {
	/* ******************************************************************** */
	.bootloader ORIGIN(FLASHCU) : 
	{
		KEEP(*(.boot2));
		KEEP(*(.boot2.*));
	} > FLASHCU

	. = ORIGIN(FLASHCU) + 256;



	/* ******************************************************************** */
	.vector_table ORIGIN(FLASHCU) + 0x100 : 
	{
		/* Core 0 vector table. */
		__svt0 = .;

		LONG( ORIGIN(SRAM4) + 4096 );
		KEEP(*(.vectortable.Reset0));
		KEEP(*(.vectortable.int0));

		__evt0 = .;



		/* Core 1 vector table. */
		__svt1 = .;

		LONG( ORIGIN(SRAM4) + 2048 );
		KEEP(*(.vectortable.Reset1));
		KEEP(*(.vectortable.int1));

		__evt1 = .;
	} > FLASHCU



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
	} > FLASHCU
	. = ALIGN(4);



	/* ******************************************************************** */
	.rodata : ALIGN(4)
	{
		*(.rodata .rodata.*);
		. = ALIGN(4);
	} > FLASHCU



	/* DATA section. */
	/* ******************************************************************** */
	.data : ALIGN(4)
	{
		. = ALIGN(4);

		__sdata = .;

		*(.data .data.*);

		. = ALIGN(4);
	} > SRAM0 AT >FLASHCU

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
	} > SRAM1

	. = ALIGN(4);
	__ebss = .;



	/* UNINIT section. */
	/* ******************************************************************** */
	. = ALIGN(4);
	__suninit = .;

	.uninit (NOLOAD) : ALIGN(4)
	{
		. = ALIGN(4);

		*(.uninit .uninit.*);

		. = ALIGN(4);
	} > SRAM1

	. = ALIGN(4);
	__euninit = .;




	/* System DATA section. */
	/* ******************************************************************** */
	.sysdata : ALIGN(4)
	{
		. = ALIGN(4);

		__ssysdata = .;

		*(.systemdata .systemdata.*);

		LONG(PINLOCK);

		. = ALIGN(4);
	} > SRAM5 AT >FLASHCU

	. = ALIGN(4);

	__esysdata = .;
	__lsysdata = LOADADDR(.sysdata);


	/* System BSS section. */
	/* ******************************************************************** */
	. = ALIGN(4);
	__ssysbss = .;

	.sysbss (NOLOAD) : ALIGN(4)
	{
		. = ALIGN(4);

		*(.systembss .systembss.*);

		. = ALIGN(4);
	} > SRAM5

	. = ALIGN(4);
	__esysbss = .;


	/* Discarded sections. */
	/DISCARD/ :
	{
		*(.ARM.exidx);
		*(.ARM.exidx.*);
		*(.ARM.extab.*);
	}
}
