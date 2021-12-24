// dev/drivers/pic8259.rs
//
// This module provides support for the 8259 PIC.


#![no_std]

use x86_64::instructions::port::Port;

// Sends the command to initialize the PIC.
const INIT_CMD: u8 = 0x11;
// Sends a command to acknowledge an interrupt has occurred.
const INTREND_CMD: u8 = 0x20;
// Sets the mode for the PIC to run in.
const MODE_8086: u8 = 0x01;


// A struct to handle individual PICs. This struct is not exported, as it is only accessed through another struct.
struct PIC
{
	offset: u8,
	cmd: Port<u8>,
	
