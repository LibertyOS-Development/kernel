// src/libcore/sys/mod.rs
//
// This is the mod.rs file for the libcore::sys module.


// Booting the kernel, bootloader support, etc.
pub mod boot;

// Console definitions/functionality
pub mod console;

// CPU
pub mod cpu;

// Global descriptor table (GDT)
pub mod gdt;

// Interrupt descriptor table (IDT)
pub mod idt;

// Logger
pub mod log;

// PCI
pub mod pci;

// System processes
pub mod proc;

// Manages prompts
pub mod prompt;

// Random number generation
pub mod rand;

// Root system descriptor table
pub mod rsdp;

// Syscalls
pub mod sc;



