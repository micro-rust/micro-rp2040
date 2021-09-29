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
	SRAM4 (RWX) : ORIGIN = 0x2004000, LENGTH = 4K
	SRAM5 (RWX) : ORIGIN = 0x2004100, LENGTH = 4K

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
	.bootloader ORIGIN(FLASH) : 
	{
		KEEP(*(.boot2));
		KEEP(*(.boot2.*));
	} > FLASH

	. = ORIGIN(FLASH) + 256



	/* ******************************************************************** */
	.vectortable ORIGIN(FLASH + 0x100) : 
	{
		/* Core 0 vector table. */
		__svt0 = .;

		LONG( ORIGIN(SRAM4) + 4096 );
		KEEP(*(.vectortable.Reset0))
		KEEP(*(.vectortable.int0));

		__evt0 = .;



		/* Core 1 vector table. */
		__svt1 = .;

		LONG( ORIGIN(SRAM4) + 2048 );
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
	.rodata (NOLOAD) : ALIGN(4)
	{
		*(.rodata .rodata.*);
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
	} > SRAM1 AT FLASH

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

		*(.sysdata .sysdata.*);

		. = ALIGN(4);
	} > SRAM5 AT FLASH

	. = ALIGN(4);

	__esysdata = .;
	__lsysdata = LOADADDR(.sysdata);


	/* System BSS section. */
	/* ******************************************************************** */
	. = ALIGN(4);
	__ssysbss = .;

	.bss (NOLOAD) : ALIGN(4)
	{
		. = ALIGN(4);

		*(.sysbss .sysbss.*);

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
