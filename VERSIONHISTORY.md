Version History

0.11.4
 - Added fat32 as a dependency (Cargo.toml).
 - Wrote a basic implemention for BlockDevice (src/lib.rs).
 - Uncommented the time dependency (Cargo.toml).
 - Updated kernel version (src/main.rs).
 - Updated version number (Cargo.toml).

0.11.3
 - Added the bytesize module (src/bytesize.rs).
 - Added a header to the bytesize module, to explain what the module does (src/bytesize.rs).
 - Imported the bytesize module (src/lib.rs).
 - Updated the kernel version (src/main.rs).
 - Updated the version number (Cargo.toml).
 - Created the byteparse module (src/byteparse.rs).
 - Imported the byteparse module (src/lib.rs).

0.11.2
 - Added uefi as a dependency (Cargo.toml).
 - Added the #![feature(abi_efiapit)] line (src/main.rs).
 - Imported uefi::prelude::* (src/main.rs).
 - Imported uefi::ResultExt (src/main.rs).
 - Updated the kernel version (src/main.rs).
 - Updated the version number (Cargo.toml).
 - Created the ps2mouse directory (src/libraries/ps2mouse).
 - Added ps2mouse as a dependency (Cargo.toml).
 - Commented out the newly-added dependency [ps2mouse] as some bugs are being fixed (Cargo.toml).
 - Added the #![allow(unused_imports)] line (src/main.rs).
 - Merged a pull request that improved performance of a created vector. This change was identified by Reddit user /u/NateReinarWoodwind, and the pull-request was made by GitHub user @mycielski. Thank you both for your contributions! (src/main.rs).
 - Added the #![allow(unused_mut)] line (src/main.rs).

0.11.1
 - Created the io module (src/libraries/io).
 - Began writing the error handling module for the io module (src/libraries/io/err.rs).
 - Updated the kernel version (src/main.rs).
 - Updated the version number (Cargo.toml).
 - Commented out an unsafe function that would print "New!" to the screen (src/main.rs).
 - Updated the list of features (README.md).
 - Updated the portion of the README that describes the expected behavior of the kernel (README.md).
 - Added the readme section to the manifest (Cargo.toml).
 - Alphabetized the sections of the manifest (Cargo.toml).
 - Added the keywords section to the manifest (Cargo.toml).
 - Added the categories section to the manifest (Cargo.toml).
 - Slightly modified the description in the manifest (Cargo.toml).
 - Created a new test, but the test does not compile. This test should be fixed in the next update (tests/heapalloc.rs).

0.11.0
 - Imported the alloc crate (src/lib.rs).
 - Created the allocator module (src/allocator.rs).
 - Imported the allocator module (src/lib.rs).
 - Added the #![allow(dead_code)] line to the intr module (src/intr.rs).
 - Added the #![allow(deprecated)] line to the gdt module (src/gdt.rs).
 - Added the #![feature(alloc_error_handler)] line to src/lib.rs (src/lib.rs).
 - Added a basic alloc error handler (src/lib.rs).
 - Imported the alloc crate (src/main.rs).
 - Imported alloc::boxed::Box (src/main.rs).
 - Added code to the kernel_main function (src/main.rs).
 - Imported various portions of the x86_64 crate (src/allocator.rs).
 - Created the init_heap function (src/allocator.rs).
 - Imported the allocator module into kernel_main (src/main.rs).
 - Added some code to make use of the newly created heap (kernel_main, src/main.rs).
 - Added the #![allow(unused_variables)] line (src/main.rs).
 - Updated the kernel version (src/main.rs).
 - Updated the version number (Cargo.toml).
 - Added linked_list_allocator as a dependency (Cargo.toml).
 - Imported the linked_list_allocator crate (src/allocator.rs).
 - Implemented portions of linked_list_allocator in the kernel (src/allocator.rs).
 - Added bitflags as a dependency (Cargo.toml).
 - Imported the ctypes module (src/lib.rs).
 - Uncommented portions of lib.rs that were commented out. The portions in question were for some functionality that required the alloc crate (src/lib.rs).

0.10.0
 - Commented out a line that established the entry_point for the kernel [entry_point!], which was already defined by src/main.rs. This conflict resulted in an error when running the "cargo test" command (src/lib.rs).
 - Completely rewrote main.rs. I have yet to write documentation in the form of comments, but the actual code is formatted in a way for documentation to be easily integrated at a later date (src/main.rs).
 - Completely rewrote the mem module. As opposed to main.rs, the mem module does have some basic documentation, but better documentation is still needed (src/mem.rs).
 - Changed the #![warn(dead_code)] line to #![allow(dead_code)] (src/main.rs).
 - Changed the #![warn(deprecated)] line to #![allow(deprecated)] (src/main.rs).
 - Changed the #![warn(unused_features)] line #![allow(unused_features)] (src/main.rs).
 - Added the #![allow(dead_code)] line (src/gdt.rs).
 - Added the #![allow(deprecated)] line (src/gdt.rs).
 - Added the #![allow(unused_features)] line (src/gdt.rs).
 - Added the #![allow(dead_code)] line (src/intr.rs).
 - Added the #![allow(deprecated)] line (src/intr.rs).
 - Added the #![allow(unused_features)] line (src/intr.rs).
 - Added the #![allow(dead_code)] line (src/mem.rs).
 - Added the #![allow(deprecated)] line (src/mem.rs).
 - Added the #![allow(unused_features)] line (src/mem.rs).
 - Imported x86_64::structures::paging::OffsetPageTable (src/mem.rs).
 - Updated the kernel verison (src/main.rs).
 - Updated the version number (Cargo.toml).

0.9.7
 - Added a TODO message about adding messages for stages 1 and 2 (src/main.rs).
 - Removed a redundant import of x86_64::structures::paging::PageTable (src/main.rs).
 - Added some code to translate the addresses of page-tables to a more readable format (src/main.rs).
 - Moved the startup messages to be above the messages about page-tables and addresses and whatnot (src/main.rs).
 - Added the #![warn(dead_code)] line (src/main.rs).
 - Added the #![warn(deprecated)] line (src/main.rs).
 - Added the #![warn(unused_features)] line (src/main.rs).
 - Updated the kernel version (src/main.rs).
 - Updated the version number (Cargo.toml).

0.9.6
 - Upon some reflection, the decision to integrate the bootloader into the kernel was a mistake. The bootloader is not necessary for the kernel to function, so it does not make sense to include it in the kernel directory, as doing so would only slow down the kernel/compile time.
 - Removed the boot directory (src/libraries/boot)
 - Updated the kernel version (src/main.rs).
 - Updated the version number (Cargo.toml).
 - The new version of the vgabuff module is not working as it should, so I have replaced it with a version that was written by @phil-opp (src/vgabuff.rs).
 - Created the rewrite directory (rewrite).
 - While the issues with the aforementioned vgabuff module are being worked out, the rewritten version has been moved to the rewrite directory (rewrite).

0.9.5
 - Updated the kernel version (src/main.rs).
 - Updated the version number (Cargo.toml).
 - Created the boot directory (src/libraries).
 - Wrote the initial version of the bootinfo module of boot (src/libraries/boot/src/bootinfo.rs).
 - Created the asm directory (src/libraries/boot/src/asm).
 - Wrote the initial version of e820.s (src/libraries/boot/src/asm/e820.s).
 - Wrote a good amount of the code for the initial version of stg1.s (src/libraries/boot/src/asm/stg1.s)

0.9.4
 - Updated the README's list of WIP features (README.md).
 - Updated the README's list of implemented features (README.md).
 - Updated the kernel version (src/main.rs).
 - Updated the version number (Cargo.toml).
 - Added the AsSlice trait (src/lib.rs).
 - Added the AsMutSlice trait (src/lib.rs).
 - Added the StableDeref trait (src/lib.rs).
 - Added (and commented out) additional functionality for the StableDeref trait. Until a global allocator is introduced to the kernel, the code in question shall remain commented out (src/lib.rs).

0.9.3
 - Due to the previous update modifying the project structure of the kernel, the logo file was no longer included in the README. This issue has been addressed (addressing said issue is the sole point of this version).
 - Updated the version number (Cargo.toml).
 - Updated the kernel version (src/main.rs).

0.9.2
 - Created a basic file for handling ctypes (src/ctypes.rs).
 - Created the libraries directory (src/libraries).
 - Created the utilities directory (src/libraries/utilities).
 - Created a blank mod.rs file (src/libraries/mod.rs).
 - Moved the graphics directory into the libraries directory (src/libraries).
 - Created the runtime directory (src/libraries/runtime).
 - Created a Dockerfile (Dockerfile). This does not work yet.
 - Created a .dockerignore file (.dockerignore).
 - Added a line that installs the bootimage crate (bootstrap.sh).
 - Changed test_main() to testmain() (src/main.rs).
 - Changed test_main() to testmain() (src/lib.rs).
 - Updated the version number (Cargo.toml).
 - Updated the kernel version (src/main.rs).
 - Complete overhaul of the vgabuff module (src/vgabuff.rs). This includes better formatting, documentation, etc.

0.9.1
 - Added some basic instructions to compile the kernel and run it with QEMU (README.md).
 - Created a script to automate the process of configuring the system to build and run LibertyOS (bootstrap.sh).
 - Updated the version number (Cargo.toml).
 - Updated the kernel version (src/main.rs).
 - Created a file to keep track of people who are working on this project (doc/Contributors).
 - Removed file (triplet)

0.9.0 - PAGING
 - Enabled the "map_physical_memory" feature of the bootloader crate (Cargo.toml).
 - Updated the version number (Cargo.toml).
 - Updated the kernel version (src/main.rs).
 - Imported the bootloader crate (src/main.rs).
 - Added the bootinfo argument to the _start function (src/main.rs).
 - Added the entry_point macro (src/main.rs).
 - Changed the _start function to kernmain (src/main.rs).
 - Imported the bootloader crate (src/lib.rs).
 - Implemented an entry-point for cargo test (test_kernmain, src/lib.rs).
 - Used the entry_point macro to use the newly-created test_kernmain (src/lib.rs).
 - Created the mem module (src/mem.rs).
 - Imported the mem module (src/lib.rs).
 - Imported the x86_64 crate (src/mem.rs).
 - Created the active_lvl4tab function (src/mem.rs).
 - Imported libertyos_kernel::mem::active_lvl4_tab into the kernmain function (src/main.rs).
 - Fixed a missing component of kernmain that invokes the test_main function (src/main.rs).
 - Created the translate_address function (src/mem.rs).

0.8.0 - PAGE-FAULT HANDLING
 - Removed a random slash mark that I mistakenly added to the beggining of the VERSIONHISTORY file.
 - Updated the version number (Cargo.toml).
 - Updated the kernel version (src/main.rs).
 - Implemented a basic handler for page-faults (src/intr.rs).

0.7.2
 - LibertyOS's kernel now has full support for the 104-key US keyboard.
 - Added some code to the keyboard_interrupt_handler (src/intr.rs).
 - Added the pc_keyboard crate (Cargo.toml).

0.7.1
 - Updated the version number (Cargo.toml).
 - Updated the kernel version (src/main.rs).
 - Removed a line in _print that caused everything printed to the VGA-buffer to be duplicated (src/vgabuff.rs).
 - Changed the color of the text to red (src/vgabuff.rs).
 - Added a blank line, after the kernel version, to give the user some room to type (src/main.rs).

0.7.0 - KEYBOARD SUPPORT, MINOR OPTIMIZATIONS
 - Commented out a test-case that causes the testing to hang (test_println_many, src/vgabuff.rs).
 - Added a copy of the current license to the root directory of the repository.
 - Created the hltloop function (src/lib.rs).
 - Removed commented-out imports (src/main.rs).
 - Removed some commented-out lines from a failed attempt to print the LibertyOS logo upon startup (src/main.rs).
 - Replaced loop with libertyos_kernel::hltloop (_start, src/main.rs).
 - Replaced loop with libertyos_kernel::hltloop (panic, src/main.rs).
 - Replaced loop with the hltloop function (_start, src/lib.rs).
 - Replaced loop with the hltloop function (test_panic_handler, src/lib.rs).
 - Condensed several imports into a single line of code (src/intr.rs).
 - Removed a comment that reminded me to condense the imports (see above) (src/intr.rs).
 - Implemented keyboard_interrupt_handler (src/intr.rs).
 - Condensed the code of timer_interrupt_handler (src/intr.rs).
 - Added "Keyboard" to IntrIdx (src/intr.rs).
 - Added support for the number keys. I know that it would be very difficult to write in just numbers (although, you could still use binary), but the fact that the kernel can now detect keystrokes is awesome.

0.6.0 - HARDWARE INTERRUPTS
 - Added support for pic8259 (Cargo.toml).
 - Made a new .gitignore file.
 - Added /target/ (.gitignore).
 - Added Cargo.lock (.gitignore).
 - Imported the spin crate (src/intr.rs).
 - Importerd the pic8259 crate (src/intr.rs).
 - Updated the kernel version that is displayed (src/main.rs).
 - Added code to init (src/lib.rs), to initialize PICS (src/intr.rs).
 - Enabled hardware interrupts (src/lib.rs).
 - Removed the line about no errors being detected, as I have just learned that said message is displayed, even if errors occur (src/main.rs).
 - Rewrote the test_println_output test (src/vgabuff.rs).
 - Added some code to avoid deadlocks (src/vgabuff.rs).
 NOTE: The tests have been hanging on the test_println_many test-case, but this should be addressed in the next update.
 NOTE: The welcome messages may be printed more than once. This is going to be addressed in the next version.

0.5.4
 - Updated the version number (src/main.rs).
 - Added embedded-graphics to Cargo.toml.
 - Added tinybmp to Cargo.toml.

0.5.3
 - Added the stackoverflow test.

0.5.2
 - I really wish I did not need to update the version number, just to add a README, but I needed a README, so I had to put out a pointless update.

0.5.1
 - Added a basic global descriptor table implementation (src/gdt.rs).
 - Added an init function to handle loading the newly-added GDT (src/gdt.rs).
 - Added a line of code, to the existing init function, to load the init function of gdt (src/lib.rs).
 - Updated the version number (Cargo.toml).
 - Updated the version number (src/main.rs).

0.5.0 - HANDLING DOUBLE-FAULTS
 - Removed a line (_start, src/main.rs) that caused an intentional breakpoint exception.
 - Added notes to major versions of the kernel.
 - Added a function to handle double-fault exceptions (src/intr.rs).
 - Added a comment to identify the breakpoint exception handler (src/intr.rs).
 - Added a comment to identify the double-fault exception handler (src/intr.rs).
 - Added support for the gdt module (src/lib.rs).
 - Created the gdt module.
 - Created a basic task state segment (TSS) (src/gdt.rs).
 - Removed an unneccesary line that referenced the "tests" module, despite said module not existing (src/lib.rs).

0.4.2
 - Given the fact that the time crate has not been implemented yet, it was commented out in main.rs, and in Cargo.toml.
 - Gutted the majority of code for the idtinit function (src/intr.rs).
 - Imported lazy_static into src/intr.rs.
 - Implemented an IDT, using lazy_static.
 - Added a function (init), to src/lib.rs, which uses src/intr.rs to handle interrupts.
 - Introduced the init function to the _start function of src/main.rs.
 - Removed several commented out lines, from a failed attempted to draw ASCII art. The lines in question were in the _start function on src/main.rs.
 - Removed the message that was displayed when the kernel was loaded successfully.
 - Introduced a breakpoint exception to the _start function of src/main.rs.
 - Added a message to notify the user of the kernel preventing a crash, due to the newly-added breakpointe exception (see _start, src/main.rs).
 - Implemented a call to init, in the test _start function of src/lib.rs.
 - Added a test-case to src/intr.rs (test_breakpnt_exc).

0.4.1
 - Added support for the time crate, but said crate has not been put to any use, as of now.
 - Removed some notes that were cluttering up the Cargo.toml file.

0.4.0 - BASIC HANDLING OF CPU EXCEPTIONS
 - Implemented a basic method for handling CPU exceptions (see src/intr.rs).
 - Enabled the "abi_x86_interrupt" feature.

0.3.3
 - Fixed the issue that would cause the kernel to panic when running the basicboot test.
 - Added the shouldfail test-case to the shouldpanic component of the tests library.
 - Removed the test-harness for the shouldpanic test.

0.3.2
 - Renamed the name of the crate to "libertyos_kernel".
 - Created a new library to handle tests.
 - Added the basicboot test.
 - Removed a TODO note from main.rs, about fixing the issues that prevented the kernel from compiling for testing.
 - Created a lib.rs file, with some basic code to handle the kernel's ever-expanding code.
 - Removed some code from main.rs, as lib.rs now handles part of what main.rs used to handle.
 - Integrated the tests crate into the kernel's code, so that the whole thing compiles. The tests panic, but that should be fixed soon.

0.3.1
 - Added a rust-toolchain file, so that the nightly version of Rust is used by default.
 - Commented out a test-case that was preventing the kernel from compiling.

0.3.0 - BASIC TESTING
 - Removed the [profile.dev] section from Cargo.toml.
 - Removed the [profile.release] section from Cargo.toml.
 - Added the CanTest trait to src/main.rs.
 - Added an implementation of the CanTest trait, to src/main.rs.
 - Added the test_simple_println test-case.
 - Renamed the trivassert test case to test_trivassert, so that the test-cases are easily identifiable.
 - Removed the previous method of executing tests, found in the "for test in tests" section of the testexec function.
 - Implemented CanTest's run to the testexec function.
 - Removed the text that was printed when executing the test_trivassert test-case.
 - Fixed the issues that prevented "cargo test" from compiling successfully. The included tests should work as intended.

0.2.9
 - Added an key to the [package.metadata.bootimage], so that QEMU exits after five minutes of running the trivassert test case.

0.2.8
 - Configured Cargo.toml, so that when running tests (using QEMU), information is output to serial, rather than to the VGA buffer, in a seperate window.
 - Modified the formatting used in the "test-args" attribute of the [package.metadata.bootimage] section. The changes in question serve no functional purpose, but make the file look nicer. :) 
 - Created a separate panic-handler for booting the kernel in testing/debug mode.
 - Created an attribute, for what was the sole panic-handler, so that the kernel uses the original panic-handler, rather than the panic-handler that should be used when running tests in QEMU.

0.2.7
 - Configured bootimage's arguments, in Cargo.toml, so that QEMU prints messages to stdout, in addition to being output to the VGA buffer.

0.2.6
 - Modified the trivassert test case, so that it uses the serprint and serprintln macros
 - Added some details to the Cargo.toml, so that the kernel can be published to crates.io (It has not been published, but should be in the near future)

0.2.5
 - Removed a TODO note, about removing unsafe function in main.rs
 - Added a TODO task, to fix the issues that prevent kernel tests from compiling

0.2.4
 - Added serprint macro
 - Added serprintln macro
 - Added an expect message for the _print function of the ser module

