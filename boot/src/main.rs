#![feature(restricted_std)]
use std::process::Command;
use bootloader_locator::locate_bootloader;

pub fn main()
{
	let bl_manifest = locate_bootloader("bootloader").unwrap();
	dbg!(bootloader_manifest);
}
