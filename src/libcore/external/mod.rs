// external/mod.rs

pub mod compmem;
pub mod cpmem;
pub mod movemem;
pub mod setmem;

const WSIZE: usize = core::mem::size_of::<usize>();
