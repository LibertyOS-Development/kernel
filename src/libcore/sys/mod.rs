// src/libcore/sys/mod.rs
//
// This is the mod.rs file for the libcore::sys module.


// Console definitions/functionality
pub mod console;

// CPU
pub mod cpu;

// Global descriptor table (GDT)
pub mod gdt;

// Interrupt descriptor table (IDT)
pub mod idt;

// System processes
pub mod proc;

// Manages prompts
pub mod prompt;

// Random number generation
pub mod rand;

// Syscalls
pub mod sc;



