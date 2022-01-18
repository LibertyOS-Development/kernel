// src/libcore/sys/cpu.rs
//
// Get information about CPU.

/*
	IMPORTS
*/

use raw_cpuid::CpuId;

use crate::serprintln;


// Initialization
pub fn init()
{
	// Create a new CPUID
	let cpuid = CpuId::new();

	if let Some(vinfo) = cpuid.get_vendor_info()
	{
		serprintln!("[INFO] CPU: {}\n", vinfo);
	}


	if let Some(proc_brandstr) = cpuid.get_processor_brand_string()
	{
		serprintln!("[INFO] CPU: {}\n", proc_brandstr.as_str().trim());
	}

	if let Some(proc_freqinfo) = cpuid.get_processor_frequency_info()
	{
		let proc_basefreq = proc_freqinfo.processor_base_frequency();
		serprintln!("[INFO] CPU: {} MHz\n", proc_basefreq);
	}
}
