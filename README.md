![LibertyOS Logo](https://raw.githubusercontent.com/LibertyOS-Development/kernel/main/resources/images/bmp/Logo-Dark.bmp)

This is the official repository of the LibertyOS kernel. LibertyOS is an operating system, built with Rust, that is open-source, free-to-use, and open to new contributors. Everything in LOS is, or will be, built from scratch. Currently, LOS supports x86_64 systems, and can be booted on real hardware, using a simple bootloader. This operating system is being worked on continuously, and all changes are logged in the VERSIONHISTORY.md file (in the root directory of this repository). If you want to see a more detailed explanation of changes between versions, you can read the commits on the GitHub repository.

## Building/Running LibertyOS
1. Clone this repository.
2. Navigate to the location that you cloned this repository to.
3. Ensure you have installed the following:
	- cURL
	- QEMU
4. Run the bootstrap script (./bootstrap.sh)
5. Compile the kernel with "cargo build --release", or run the kernel in QEMU, using "cargo run --release".


#### CURRENTLY IMPLEMENTED FEATURES
 - Support for x86-64 architecture
 - Support for Rust's core and alloc crates
 - Memory allocation, using linked-lists, buddy-allocation, and bump-allocation
 - Basic error-handling
 - Built-in support for running LOS with QEMU
 - Support for several keyboard layouts
 - Basic support for asynchronous functions, including a basic task-executor
 - Support for reading/writing to filesystems (support for specific filesystems has yet to be implemented, but the foundations have been implemented already)
 - Uses a core library, built for LibertyOS (libcore)
 - Supports mathematical calculations, including basic algebra, geometry, etc.
 - Support for C-types
 - Basic support for parsing fonts
 - Time-keeping
 - System-calls
 - Basic process-management
 - Support for stdin, stdout, and stderr
 - Basic support for running processes in userspace


#### WIP FEATURES
 - A basic filesystem, built for LibertyOS
 - Full documentation for the entire kernel
 - Full support for reading/writing to FAT filesystems
 - Basic networking capabilities
 - Support for computer mice
 - A basic shell


#### PLANNED FEATURES
 - Support for ARM-based targets
 - Support for glibc/musl (or, perhaps, rewriting certain parts of them)
 - Support for FAT, FAT32, EXFAT filesystems
 - Support for ext2, ext3, ext4 filesystems
 - Support for NTFS filesystem
 - Hostnames
 - Ability to update the kernel from within the operating system
 - Support for non-English characters
 - Full support for Unicode
 - Basic customization

#### CURRENTLY EXPECTED BEHAVIOUR
 - The kernel initializes the critical portions of LibertyOS
 - The kernel displays a message about the setup process
 - The kernel encounters a page-fault. :(
