// src/dev/drivers/uart.rs
//
// This module provides the kernel with basic access to UART devices.

use bitflags::bitflags;

// These are interrupt-enable flags.
bitflags!
{
	struct IntrEnFlag: u8
	{
		const 	RCVD = 1;
		const 	SENT = 1 << 1;
		const 	ERR = 1 << 2;
		const	STATCHANGE = 1 << 3;
	}
}

// These are line-status flags.
bitflags!
{
	pub struct LineStatFlag: u8
	{
		const INPFULL = 1;
		const OUTPEMPTY = 1 << 5;
	}
}
