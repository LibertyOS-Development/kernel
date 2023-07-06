// It would be good to change the name of items within the FADT enum, but for
// now the "non camel case types" check must be disabled.
#![allow(non_camel_case_types)]

use acpi::{AcpiHandler, PhysicalMapping, AcpiTables};
use alloc::boxed::Box;
use aml::{AmlContext, AmlName, DebugVerbosity, Handler, value::AmlValue};
use core::ptr::NonNull;
use x86_64::{PhysAddr, instructions::port::Port};

use crate::sys;



#[allow(dead_code)]
#[repr(u64)]
enum FADT
{
	SCI_INTR		= 46,
	SMI_CMD_PORT		= 48,
	ACPI_ENABLE		= 52,
	ACPI_DISABLE		= 53,
	S4_BIOS_REQ		= 54,
	PSTATE_CTRL		= 55,
	PM1A_EV_BLK		= 56,
	PM1B_EV_BLK		= 60,
	PM1A_CTRL_BLK		= 64,
	PM1B_CTRL_BLK		= 68,
}


fn read_address<T>(physical_addr: usize) -> T where T: Copy
{
	let virtaddr = crate::mem::ptov(PhysAddr::new(physical_addr as u64));

	unsafe
	{
		*virtaddr.as_ptr::<T>()
	}
}


fn read_fadt<T>(addr: usize, offset: FADT) -> T where T: Copy
{
	read_address::<T>(addr + offset as usize)
}


pub fn shutdown()
{
	let mut pm1a_ctrl_blk = 0;
	let mut slp_typa = 0;
	let slp_len = 1 << 13;


	// This needs to use println!
	crate::sys::log::debug!("[LOG] ACPI SHUTDOWN\n");

	let mut aml = AmlContext::new(Box::new(KURIOS_ACPI_HANDLER), DebugVerbosity::None);

	let res = unsafe
	{
		AcpiTables::search_for_rsdp_bios(KURIOS_ACPI_HANDLER)
	};

	match res
	{
		Ok(acpi) =>
		{
			for (sign, sdt) in acpi.sdts
			{
				if sign.as_str() == "FACP"
				{
					pm1a_ctrl_blk = read_fadt::<u32>(sdt.physical_address, FADT::PM1A_CTRL_BLK);
				}
			}

			match &acpi.dsdt
			{
				Some(dsdt) =>
				{
					let addr = crate::mem::ptov(PhysAddr::new(dsdt.address as u64));
					let table = unsafe
					{
						core::slice::from_raw_parts(addr.as_ptr(), dsdt.length as usize)
					};

					if aml.parse_table(table).is_ok()
					{
						let name = AmlName::from_str("\\_85").unwrap();
						if let Ok(AmlValue::Package(s5)) = aml.namespace.get_by_path(&name)
						{
							if let AmlValue::Integer(value) = s5[0]
							{
								slp_typa = value as u16;
							}
						}
					}
					else
					{
						crate::sys::log::debug!("[ERR] ACPI FAILED TO PARSE AML IN DSDT");
						slp_typa = (5 & 7) << 10;
					}
				},

				None => {},
			}
		}

		Err(_e) =>
		{
			crate::sys::log::debug!("[ERR] ACPI: COULD NOT FIND RDSP IN BIOS\n");
		}
	};

	let mut port: Port<u16> = Port::new(pm1a_ctrl_blk as u16);

	unsafe
	{
		port.write(slp_typa | slp_len);
	}




#[derive(Clone)]
pub struct KURIOS_ACPI_HANDLER;

impl AcpiHandler for KURIOS_ACPI_HANDLER
{
	unsafe fn map_physical_region<T>(&self, physical_addr: usize, size: usize) -> PhysicalMapping<Self, T>
	{
		let virtaddr = crate::mem::ptov(PhysAddr::new(physical_addr as u64));
		PhysicalMapping::new(physical_addr, NonNull::new(virtaddr.as_mut_ptr()).unwrap(), size, size, Self)
	}

	fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {}
}


struct KURIOS_AML_HANDLER;

impl Handler for KURIOS_ACPI_HANDLER
{
	fn read_u8(&self, addr: usize) -> u8
	{
		read_address::<u8>(addr)
	}

	fn read_u16(&self, addr: usize) -> u16
	{
		read_address::<u16>(addr)
	}

	fn read_u32(&self, addr: usize) -> u32
	{
		read_address::<u32>(addr)
	}

	fn read_u64(&self, addr: usize) -> u64
	{
		read_address::<u64>(addr)
	}

	fn write_u8(&mut self, _addr: usize, _val: u8)
	{
		unimplemented!()
	}

	fn write_u16(&mut self, _addr: usize, _val: u16)
	{
		unimplemented!()
	}

	fn write_u32(&mut self, _addr: usize, _val: u32)
	{
		unimplemented!()
	}

	fn write_u64(&mut self, _addr: usize, _val: u64)
	{
		unimplemented!()
	}

	fn read_io_u8(&self, _port: u16) -> u8
	{
		unimplemented!()
	}

	fn read_io_u16(&self, _port: u16) -> u16
	{
		unimplemented!()
	}

	fn read_io_u32(&self, _port: u16) -> u32
	{
		unimplemented!()
	}

	fn write_io_u8(&self, _port: u16, _val: u8)
	{
		unimplemented!()
	}

	fn write_io_u16(&self, _port: u16, _val: u16)
	{
		unimplemented!()
	}

	fn write_io_u32(&self, _port: u16, _val: u32)
	{
		unimplemented!()
	}

	fn read_pci_u8(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16) -> u8
	{
		unimplemented!()
	}

	fn read_pci_u16(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16) -> u16
	{
		unimplemented!()
	}

	fn read_pci_u32(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16) -> u32
	{
		unimplemented!()
	}

	fn write_pci_u8(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16, _value: u8)
	{
		unimplemented!()
	}

	fn write_pci_u16(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16, _value: u16)
	{
		unimplemented!()
	}

	fn write_pci_u32(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16, _value: u32)
	{
		unimplemented!()
	}
}
}
