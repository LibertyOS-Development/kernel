// src/libcore/data/ucs2.rs
//
// Support for UCS-2 encoding.

/*
	IMPORTS
*/

use bit_field::BitField;


// Err enumeration
#[derive(Debug, Copy, Clone)]
pub enum Err
{
	// Buffer overflow
	BOverflow,

	// Multi-byte
	MByte,
}


// Result type
type Result<T> = core::result::Result<T, Err>;


// Decode UCS-2 into UTF-8
pub fn dec(input: &[u16], output: &mut [u8]) -> Result<usize>
{
	let bsize = output.len();
	let mut i = 0;

	dec_w(input, |bytes|
	{
		if bytes.len() == 1
		{
			if i >= bsize
			{
				return Err(Err::BOverflow);
			}

			output[i] = bytes[0];

			i += 1;
		}
		else if bytes.len() == 2
		{
			if i + 1 >= bsize
			{
				return Err(Err::BOverflow);
			}

			output[i] = bytes[0];
			output[i + 1] = bytes[1];

			i += 2;
		}
		else if bytes.len() == 3
		{
			if i + 2 >= bsize
			{
				return Err(Err::BOverflow);
			}

			output[i] = bytes[0];
			output[i + 1] = bytes[1];
			output[i + 2] = bytes[2];

			i += 3;
		}
		else
		{
			unreachable!("[ERR] > 3 BYTES SPECIFIED FOR UCS-2 CHARACTER");
		}

		Ok(())
	})
}


// Use a custom callback to encode UCS-2 into UTF-8
pub fn dec_w<F>(input: &[u16], mut output: F) -> Result<usize>
where
	F: FnMut(&[u8]) -> Result<()>,
{
	let mut written = 0;
	for ch in input.iter()
	{
		if (0x000..0x0080).contains(ch)
		{
			output(&[*ch as u8])?;
			written += 1;
		}
		else if (0x0080..0x0800).contains(ch)
		{
			let first = 0b1100_0000 + ch.get_bits(6..11) as u8;
			let last = 0b1000_0000 + ch.get_bits(0..6) as u8;
			output(&[first, last])?;
			written += 2;
		}
		else
		{
			let first = 0b1110_0000 + ch.get_bits(12..16) as u8;
			let mid = 0b1000_0000 + ch.get_bits(6..12) as u8;
			let last = 0b1000_0000 + ch.get_bits(0..6) as u8;
			output(&[first, mid, last])?;
			written += 3;
		}
	}

	Ok(written)
}
		

// Encode UTF-8 into UCS-2
pub fn enc(input: &str, buffer: &mut [u16]) -> Result<usize>
{
	let bsize = buffer.len();
	let mut i = 0;

	enc_w(input, |ch|
	{
		if i >= bsize
		{
			Err(Err::BOverflow)
		}
		else
		{
			buffer[i] = ch;
			i += 1;
			Ok(())
		}

	})?;

	Ok(i)
}



// Use a custom callback to encode UTF-8 into UCS-2
pub fn enc_w<F>(input: &str, mut output: F) -> Result<()>
where
	F: FnMut(u16) -> Result<()>,
{
	let bytes = input.as_bytes();
	let len = bytes.len();
	let mut i = 0;

	while i < len
	{
		let ch;
		if bytes[i] & 0b1000_0000 == 0b0000_0000
		{
			ch = u16::from(bytes[i]);
			i += 1;
		}
		else if bytes[i] & 0b1110_0000 == 0b1100_0000
		{
			if i + 1 >= len
			{
				unsafe
				{
					core::hint::unreachable_unchecked()
				}
			}

			let a = u16::from(bytes[i] & 0b0001_1111);
			let b = u16::from(bytes[i + 1] & 0b0011_1111);
			ch = a << 6 | b;
			i += 2;
		}
		else if bytes[i] & 0b1111_0000 == 0b1110_0000
		{
			if i + 2 >= len || i + 1 >= len
			{
				unsafe
				{
					core::hint::unreachable_unchecked()
				}
			}

			let a = u16::from(bytes[i] & 0b0000_1111);
			let b = u16::from(bytes[i + 1] & 0b0011_1111);
			let c = u16::from(bytes[i + 2] & 0b0011_1111);
			ch = a << 12 | b << 6 | c;
			i += 3;
		}
		else if bytes[i] & 0b1111_0000 == 0b1111_0000
		{
			return Err(Err::MByte);
		}
		else
		{
			unsafe
			{
				core::hint::unreachable_unchecked()
			}
		}

		output(ch)?;
	}

	Ok(())
}
