// ----------------------------------------------------------------------------
// Second stage boot code
// Copyright (c) 2019-2021 Raspberry Pi (Trading) Ltd.
// SPDX-License-Identifier: BSD-3-Clause
//
// Device:      Winbond W25Q080
//              Also supports W25Q16JV (which has some different SR instructions)
//              Also supports AT25SF081
//              Also supports S25FL132K0
//
// Description: Configures W25Q080 to run in Quad I/O continuous read XIP mode
//
// Details:     * Check status register 2 to determine if QSPI mode is enabled,
//                and perform an SR2 programming cycle if necessary.
//              * Use SSI to perform a dummy 0xEB read command, with the mode
//                continuation bits set, so that the flash will not require
//                0xEB instruction prefix on subsequent reads.
//              * Configure SSI to write address, mode bits, but no instruction.
//                SSI + flash are now jointly in a state where continuous reads
//                can take place.
//              * Jump to exit pointer passed in via lr. Bootrom passes null,
//                in which case this code uses a default 256 byte flash offset
//
// Building:    * This code must be position-independent, and use stack only
//              * The code will be padded to a size of 256 bytes, including a
//                4-byte checksum. Therefore code size cannot exceed 252 bytes.
// ----------------------------------------------------------------------------








// ----------------------------------------------------------------------------
// Start of 2nd Stage Boot Code
// ----------------------------------------------------------------------------

.syntax unified
.cpu cortex-m0plus
.thumb

.section .text

// The exit point is passed in lr. If entered from bootrom, this will be the
// flash address immediately following this second stage (0x10000100).
// Otherwise it will be a return address -- second stage being called as a
// function by user code, after copying out of XIP region. r3 holds SSI base,
// r0...2 used as temporaries. Other GPRs not used.
.global _stage2_boot
.type _stage2_boot,%function
.thumb_func
_stage2_boot:
    push {lr}

    // Set pad configuration:

    // Load QSPI Pads base address into R3.
    ldr r3, =0x40020000
    // - SCLK 8mA drive, no slew limiting
    movs r0, #(2 << 4 | 1)
    str r0, [r3, #4]

    // Load current SDx configuration into R0.
    ldr r0, [r3, #8]
    // - SDx disable input Schmitt to reduce delay
    movs r1, #2
    bics r0, r1
    // Store the new configuration into all the SDx registers.
    str r0, [r3, #08]
    str r0, [r3, #12]
    str r0, [r3, #16]
    str r0, [r3, #20]


    // Load the XIP SSI base address into R3.
    ldr r3, =0x18000000

    // Disable SSI to allow further configuration.
    movs r1, #0
    str r1, [r3, #8]

    // Set baud rate
    movs r1, #4
    str r1, [r3, #0x14]

    // Set 1-cycle sample delay.
    movs r1, #1
    movs r2, #0xF0  // == 0xf0 so need 8 bits of offset significance
    str r1, [r3, r2]



// On QSPI parts we usually need a 01h SR-write command to enable QSPI mode
// (i.e. turn WPn and HOLDn into IO2/IO3)
program_sregs:

	// Set to 8 bits frame and TX and RX enabled.
    ldr r1, =(7 << 16)
    str r1, [r3, #0]

     // Enable SSI and select slave 0
    movs r1, #1
    str r1, [r3, #8]

    // Check whether SR needs updating
    movs r0, #0x35
    bl sreg
    movs r2, #0x02
    cmp r0, r2
    beq skip

    // Send write enable command
    movs r1, #0x06
    str r1, [r3, #0x60]

    // Poll for completion and discard RX
    bl wait
    ldr r1, [r3, #0x60]

    // Send status write command followed by data bytes
    movs r1, #0x01
    str r1, [r3, #0x60]
    movs r0, #0
    str r0, [r3, #0x60]
    str r2, [r3, #0x60]

    bl wait
    ldr r1, [r3, #0x60]
    ldr r1, [r3, #0x60]
    ldr r1, [r3, #0x60]

    // Poll status register for write completion
1:
    movs r0, #0x05
    bl sreg
    movs r1, #1
    tst r0, r1
    bne 1b

skip:
    // Disable SSI again so that it can be reconfigured
    movs r1, #0
    str r1, [r3, #8]



// Perform dummy read and go into continuous mode.
dummy_read:
#define CTRLR0_ENTER_XIP \
    (FRAME_FORMAT                          /* Quad I/O mode */                \
        << SSI_CTRLR0_SPI_FRF_LSB) |                                          \
    (31 << SSI_CTRLR0_DFS_32_LSB)  |       /* 32 data bits */                 \
    (SSI_CTRLR0_TMOD_VALUE_EEPROM_READ     /* Send INST/ADDR, Receive Data */ \
        << SSI_CTRLR0_TMOD_LSB)

    ldr r1, =(CTRLR0_ENTER_XIP)
    str r1, [r3, #0]

    movs r1, #0x0                    // NDF=0 (single 32b read)
    str r1, [r3, #4]

#define SPI_CTRLR0_ENTER_XIP \
    (ADDR_L << SSI_SPI_CTRLR0_ADDR_L_LSB) |     /* Address + mode bits */ \
    (WAIT_CYCLES << SSI_SPI_CTRLR0_WAIT_CYCLES_LSB) | /* Hi-Z dummy clocks following address + mode */ \
    (SSI_SPI_CTRLR0_INST_L_VALUE_8B \
        << SSI_SPI_CTRLR0_INST_L_LSB) |        /* 8-bit instruction */ \
    (SSI_SPI_CTRLR0_TRANS_TYPE_VALUE_1C2A      /* Send Command in serial mode then address in Quad I/O mode */ \
        << SSI_SPI_CTRLR0_TRANS_TYPE_LSB)

    ldr r1, =(SPI_CTRLR0_ENTER_XIP)
    ldr r2, =(0x18000000 + 0xF4)  // SPI_CTRL0 Register
    str r1, [r2]

    movs r1, #1                      // Re-enable SSI
    str r1, [r3, #8]

    movs r1, #0xEB
    str r1, [r3, #0x60]   // Push SPI command into TX FIFO
    movs r1, #0xA0   // 32-bit: 24 address bits (we don't care, so 0) and M[7:4]=1010
    str r1, [r3, #0x60]   // Push Address into TX FIFO - this will trigger the transaction

    // Poll for completion
    bl wait

// The flash is in a state where we can blast addresses in parallel, and get
// parallel data back. Now configure the SSI to translate XIP bus accesses
// into QSPI transfers of this form.

    movs r1, #0
    str r1, [r3, #8]   // Disable SSI (and clear FIFO) to allow further config

// Note that the INST_L field is used to select what XIP data gets pushed into
// the TX FIFO:
//      INST_L_0_BITS   {ADDR[23:0],XIP_CMD[7:0]}       Load "mode bits" into XIP_CMD
//      Anything else   {XIP_CMD[7:0],ADDR[23:0]}       Load SPI command into XIP_CMD
configure_ssi:
#define SPI_CTRLR0_XIP \
    (MODE_CONTINUOUS_READ                      /* Mode bits to keep flash in continuous read mode */ \
        << SSI_SPI_CTRLR0_XIP_CMD_LSB) | \
    (ADDR_L << SSI_SPI_CTRLR0_ADDR_L_LSB) |    /* Total number of address + mode bits */ \
    (WAIT_CYCLES << SSI_SPI_CTRLR0_WAIT_CYCLES_LSB) |    /* Hi-Z dummy clocks following address + mode */ \
    (SSI_SPI_CTRLR0_INST_L_VALUE_NONE          /* Do not send a command, instead send XIP_CMD as mode bits after address */ \
        << SSI_SPI_CTRLR0_INST_L_LSB) | \
    (SSI_SPI_CTRLR0_TRANS_TYPE_VALUE_2C2A      /* Send Address in Quad I/O mode (and Command but that is zero bits long) */ \
        << SSI_SPI_CTRLR0_TRANS_TYPE_LSB)

    ldr r1, =(SPI_CTRLR0_XIP)
    str r1, [r2]

    movs r1, #1
    str r1, [r3, #8]   // Re-enable SSI




// If entered from the bootrom, lr (which we earlier pushed) will be 0,
// and we vector through the table at the start of the main flash image.
// Any regular function call will have a nonzero value for lr.
check_return:
    pop {r0}
    cmp r0, #0
    beq vector
    bx r0
vector:
    ldr r0, =(0x10000000 + 0x0100)
    ldr r1, =(0xE0000000 + 0xED08)
    str r0, [r1]
    ldmia r0, {r0, r1}
    msr msp, r0
    bx r1




// Pass status read cmd into r0.
// Returns status value in r0.
sreg:
    push {r1, lr}
    // Send read command.
    str r0, [r3, #0x60]
    // Dummy byte:
    str r0, [r3, #0x60]
    
    bl wait_ssi_ready
    // Discard first byte and combine the next two
    ldr r0, [r3, #0x60]
    ldr r0, [r3, #0x60]

    pop {r1, pc}




wait:
    push {r0, r1, lr}

    // Command is complete when there is nothing left to send
    // (TX FIFO empty) and SSI is no longer busy (CSn deasserted)
1:
    ldr r1, [r3, #SSI_SR_OFFSET]
    movs r0, #SSI_SR_TFE_BITS
    tst r1, r0
    beq 1b
    movs r0, #SSI_SR_BUSY_BITS
    tst r1, r0
    bne 1b

    pop {r0, r1, pc}




.global literals
literals:
.ltorg

.end