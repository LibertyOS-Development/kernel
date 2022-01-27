// src/core/mod.rs
//
// This is the mod.rs for the core module.

// Memory allocation
pub mod allocator;

// Arch-related modules
pub mod arch;

// Data encoding, decoding, support for additional programming languages, etc.
pub mod data;

// Devices and drivers
pub mod dev;

// External functions
pub mod external;

// Libraries for working with filesystems
pub mod fs;

// TUI and GUI modules
pub mod graphics;

// I/O modules
pub mod io;

// Mathematics, measurements, and time-keeping
pub mod math;

// System utilities, processes, syscalls, etc.
pub mod sys;

//Task-execution
pub mod task;

// User applications and functions
pub mod user;
