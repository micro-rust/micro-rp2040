//! 8080 parallel interface.


// USAGE:
// Reset all SM.
// Reset the Memory to 0.
// Configure all the SM.
// Push Tlow then Thigh of the read timings to SM1 FIFO.
// Push Tlow then Thigh of the write timings to SM2 FIFO.
// Push Tlow then Thigh of the command timings to SM3 FIFO.
// Write the Load Read Timings program into the SM1 Execution port.
// Write the Load Write Timings program into the SM2 Execution port.
// Write the Load Write Timings program into the SM3 Execution port.
// Enable all SM.

// + Sending a command:
//   Write to the SM0 FIFO the number of commands - 1 (for commands this is 0).
//   Write to the SM0 FIFO the function to execute (for commands this is 3).
//   Write to the SM3 FIFO the command to be sent (this can be done before or after the previous steps).
//   Wait until completion (SM0 stalled waiting for data).

// + Sending data:
//   Write to the SM0 FIFO the number of commands - 1 (for commands this is 0).
//   Write to the SM0 FIFO the function to execute (for commands this is 3).
//   Write to the SM0 FIFO the number of data words - 1 (e.g. if sending 20 bytes write 19).
//   Write to the SM0 FIFO the function to execute (for data write this is 5).
//   Write to the SM3 FIFO the command to be sent (this can be done before or after the previous steps).
//   Write to the SM2 FIFO all the data to be sent (this can be done before or after the previous steps).

// + Receiving data:
//   Write to the SM0 FIFO the number of commands - 1 (for commands this is 0).
//   Write to the SM0 FIFO the function to execute (for commands this is 3).
//   Write to the SM0 FIFO the number of data words - 1 (e.g. if expecting 20 bytes write 19).
//   Write to the SM0 FIFO the function to execute (for data read this is 7).
//   Write to the SM3 FIFO the command to be sent (this can be done before or after the previous steps).
//   Read from the SM1 FIFO the data received (whenever it arrives).


// Load Read timings program for SM1.
// OUT Y    32
// OUT OSR  32
// JUMP 11

// Load Write timings program for SM2.
// OUT Y    32
// OUT ISR  32
// JUMP 20

// Load Write timings program for SM3.
// OUT Y    32
// OUT ISR  32
// JUMP 22



// NOTES : 
// SM2 and SM3 share wrap address.
// 6 cycles of latency between each word of the same type (e.g. Write to next Write).
// 9 cycles of latency between words of different type (e.g. Command to Write / Command to read).
// Substitute <NBITS> with the word size of choice (8, 9, 10, 12, 16, 18 bits).
// Use of DMA or interrupts is heavily encouraged for the 8080 parallel interface, as the 
// usually loose timings result in a lot of time spent idling. The interface is designed to allow
// easy DMA chaining. The data blocks for the Slave DMA are 2 u32 for configuration + n bytes transfer of data.



// SM Configuration.
// + EXECTRL
//     SIDE_EN : 1
//     OUT_STICKY : 1
//     WRAP_TOP :
//       SM0:  9
//       SM1: 18
//       SM2: 29
//       SM3: 29
//     WRAP_BOTTOM :
//       SM0:  0
//       SM1: 10
//       SM2: 19
//       SM3: 21
// + SHIFTCTRL
//     FJOIN_RX:
//       SM0: 0
//       SM1: 1
//       SM2: 0
//       SM3: 0
//     FJOIN_RX:
//       SM0: 1
//       SM1: 0
//       SM2: 1
//       SM3: 1
//     PULL_THRESH:
//     PUSH_THRESH:
//       - These two are dependant on the interface size <NBITS>.
//     IN_SHIFTDIR:
//     OUT_SHIFTDIR:
//       - These two I have not figured out yet. Ithink they depend on MSB or LSB configuration of the slave device.
//     AUTOPULL:
//       SM0: 1
//       SM1: 0
//       SM2: 1
//       SM3: 1
//     AUTOPUSH:
//       SM0: 0
//       SM1: 1
//       SM2: 0
//       SM3: 0
// + PINCTRL
//     SIDESET_COUNT:
//       SM0: 4
//       SM1: 3
//       SM2: 3
//       SM3: 3
//     OUT_COUNT:
//       SM0: X
//       SM1: X
//       SM2: 8
//       SM3: 8
//     IN_BASE:
//       SM0: X
//       SM1: The lowest pin used in the interface.
//       SM2: X
//       SM3: X
//     SIDESET_BASE:
//       - The lowest pin used for the RST, CS, RS, WR, RD signals, in that order.
//     OUT_BASE:
//       SM0: The lowest pin used in the interface.
//       SM1: X
//       SM2: The lowest pin used in the interface.
//       SM3: The lowest pin used in the interface.


// Dispatch - SM0 - 11 instructions.

-- WRAP TARGET --

// Read Count and Function pointer.
.entry
OUT X 32        // COUNT
OUT Y 32        // FUNCTION


// .execute - Instruction 2
MOV PC Y


// .command - Instruction 3.
IRQ WAIT 7 side 0b1010   // Write signal with sideset.
JMP 8                    // Jump to wait.


//.write - Instruction 5.
IRQ WAIT 6 side 0b1110   // Write signal with sideset.
JMP 8                    // Jump to wait.


// .read - Instruction 7.
IRQ WAIT 5 side 0b1101   // Read signal with sideset.


// .wait - Instruction 8
WAIT 1 IRQ 4             // Wait until Command, Write or Read signal end of word.
JMP X-- 2                // If there are still words left, jump back to execute.


--WRAP--


// Reader - SM1 - 8 instructions - Instruction 10.
// Sideset has access to RD WR pins only.
-- WRAP TARGET --
// Wait until a new job.
WAIT 1 IRQ 5

// Set Pin directions.
MOV NULL OSR
OUT PINDIRS 32

// Wait low latency.
MOV X Y
JMP X-- 14

// Read the data.
IN PINS <NBITS>

// Wait high latency.
MOV X OSR
JMP X-- 17

IRQ NOWAIT 4
--WRAP--



// Command - SM3 - 2 instructions - Instruction 19.
// Sideset has access to RD WR pins only.
// Shares WRAP address with SM 2.
-- WRAP TARGET --
WAIT 1 IRQ 7

JMP 22



// Write - SM2 - 8 instructions - Instruction 21.
// Sideset has access to RD WR pins only.
-- WRAP TARGET --
// Wait until a new job.
WAIT 1 IRQ 6

// Set Pin directions.
MOV !NULL OSR
OUT PINDIRS 32

// Output the data.
OUT PINS <NBITS>

// Wait low latency.
MOV X Y
JMP X-- 26


// Wait high latency.
MOV X ISR
JMP X-- 28 side 0b111

IRQ NOWAIT 4
--WRAP--
