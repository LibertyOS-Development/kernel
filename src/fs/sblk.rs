use crate::KSIZE;
use super::blk::Block;
use super::blkdev::BlockDevIO;
use core::convert::TryInto;

const SBLK_ADDR: u32 = (KSIZE / super::BLKSIZE) as u32;
const SIG: &[u8; 8] = b"LIBERTAS";

#[derive(Debug)]
pub struct SuperBlock
{
	sig: &'static[u8, 8],
	vers: u8,
	blksize: u32,
	pub blk_count: u32,
	pub alloc_count: u32,
}

impl SuperBlock
{
	pub fn checkata
