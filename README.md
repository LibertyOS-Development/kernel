![LibertyOS Logo](https://raw.githubusercontent.com/LibertyOS-Development/kernel/main/src/libraries/graphics/images/bmp/Logo-Dark.bmp)

This is the official repository of the LibertyOS kernel. LibertyOS is an operating system, built with Rust, that is open-source, free-to-use, and open to new contributors. Everything in LOS is, or will be, built from scratch. Currently, LOS supports x86_64 systems, and can be booted on real hardware, using a simple bootloader. This operating system is being worked on continuously, and all changes are logged in the VERSIONHISTORY.md file (in the root directory of this repository). If you want to see a more detailed explanation of changes between versions, you can read the commits on the GitHub repository.

## Building/Running LibertyOS
1. Clone this repository.
2. Navigate to the location that you cloned this repository to.
3. Ensure you have installed the following:
	- cURL
	- QEMU
4. Run the bootstrap script (./bootstrap.sh)
5. Compile the kernel with "cargo build", or run the kernel in QEMU, using "cargo run".


#### CURRENTLY IMPLEMENTED FEATURES
- Support for x86_64
- A small array of tests
- Error handlers
- Built-in support for running LOS with QEMU
- Displaying text
- Colored text
- Support for the 104-key US keyboard

#### WIP FEATURES
- Basic memory allocation
- Integrating the alloc crate
- Basic documentation

#### PLANNED FEATURES
- A simple shell
- Custom filesystem
- Support for ARM-based targets
- Support for glibc/musl (or, perhaps, rewriting certain parts of them)
- Support for FAT, FAT32, EXFAT filesystems
- Support for ext2, ext3, ext4 filesystems
- Support for NTFS filesystem
- Basic networking
- Hostnames
- Ability to update the kernel from within the operating system
- A better, nicer font
- Full support for ASCII
- System time
- Basic customization
