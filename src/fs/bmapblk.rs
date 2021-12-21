use bit_field::BitField;
use super::blk::Block;
use super::sblk;
use super::sblk::SuperBlock;

pub const BMAP_SIZE: usize = 8 * super::BLKSIZE;

pub struct BMapBlock {}

impl BMapBlock
{
	fn blkidx(addr: u32) -> u32
	{
		let sb = SuperBlock::read();
		let size = sb.blksize();
		let i = addr - sb.data_area();
		sb.bmap_area() + (i / size / 8)
	}

	fn buffidx(addr: u32) -> usize
	{
		let sb = SuperBlock::read();
		let i = (addr - sb.data_area()) as usize;
		i % sb.blksize() as usize
	}

	pub fn alloc(addr: u32)
	{
		let mut blk = Block::read(BMapBlock::blkidx(addr));
		let bmap = blk.datamut();
		let i = BMapBlock::buffidx(addr);
		if !bmap[i / 8].get_bit(i % 8)
		{
			bmap[i / 8].set_bit(i % 8, true);
			blk.write();
			sblk::inc_alloc_count();
		}
	}

	pub fn free(addr: u32)
	{
		let mut blk = Block::read(BMapBlock::blkidx(addr));
		let bmap = blk.datamut();
		let i = BMapBlock::buffidx(addr);
		bmap[i / 8].set_bit(i % 8, false);
		blk.write();
		sblk::dec_alloc_count();
	}

	pub fn next_free_addr() -> Option<u32>
	{
		let sb = SuperBlock::read();
		let size = sb.blksize();
		let n = sb.blkcount() / size / 8;
		for i in 0..n
		{
			let blk = Block::read(sb.bmap_area() + i);
			let bmap = blk.data();
			for j in 0..size
			{
				for k in 0..8
				{
					if !bmap[j as usize].get_bit(k)
					{
						let bs = BMAPSIZE as u32;
						let addr = sb.data_area() + i * bs + j * 8 + k as u32;
						return Some(addr);
					}
				}
			}
		}
		None
	}
}

pub fn freeall()
{
	let sb = SuperBlock::read();
	let a = sb.bmap_area();
	let b = sb.data_area();
	for addr in a..b
	{
		Block::new(addr).write();
	}
}
