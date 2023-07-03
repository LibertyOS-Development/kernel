use alloc::vec::Vec;


pub trait Serialize
{
	fn serialize(&self) -> Vec<u8>
	{
		let mut res = Vec::new();
		self.serialize_to(&mut res, 0, 0);
		res
	}

	fn format(&self, indent: u32) -> Vec<u8>
	{
		let mut res = Vec::new();
		self.serialize_to(&mut res, indent, 0);
		res
	}

	fn serialize_to(&self, buffer: &mut Vec<u8>, indent: u32, level: u32);

}
