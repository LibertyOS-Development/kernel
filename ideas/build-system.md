# Build System Idea - July 10, 2023 - Daniel Teberian

I have an idea for a way to handle building the kernel for different systems. Here is a basic explanation of how the build-system could work. Bear in mind that much of the features that would be required for such a system have yet to be implemented in the kernel, so consider this idea as something to do a while from now.

1. The user uses cURL (perhaps a GUI could be created) to download a script from one of our repositories.
2. The user changes the mode of the script, executes the script, and the script asks some questions from the user.
3. The script will ask if the user wishes to build the kernel for the host system. If the user chooses to build for the host system, the script will automatically determine what target-triplet needs to be used.
4. If the user wants to build the kernel for another machine, the user can specify a triplet to use.
5. The script asks the user whether it should build and set up the kernel so that the user can boot off of a given device, or if the script should just build the kernel.
6. If the user wants to have the kernel configured and installed onto their machine, the script will ask to be pointed to the partition that the user wants to install the kernel onto.
7. After the script determines what needs to be built, the script downloads rustup, configures it for the host system, and installs whatever is needed to build for the specified target.
8. The script tests the installation of Rust. If all is well, the script compiles the kernel, using the information provided at the start of the script.
9. The result should be either a bootable kernel, or a partition/drive with a bootable kernel installed onto it.
