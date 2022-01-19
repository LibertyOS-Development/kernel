#!/bin/bash -e
clear

echo " _     _ _               _          ___  ____  "
echo "| |   (_) |__   ___ _ __| |_ _   _ / _ \/ ___| "
echo "| |   | | '_ \ / _ \ '__| __| | | | | | \___ \ "
echo "| |___| | |_) |  __/ |  | |_| |_| | |_| |___) |"
echo "|_____|_|_.__/ \___|_|   \__|\__, |\___/|____/ "
echo "                             |___/             "
echo "By using this script, and by running LibertyOS, you are agreeing to the terms outlined in the LICENSE."
echo "Installing rustup..."
if ! rustc --version; then
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
else
echo "Rust already installed"
fi
echo "Installing the bootimage crate..."
cargo install bootimage
echo "Your system should now be correctly configured to build and run LibertyOS."
echo "To launch the kernel in a VM, use the cargo run command (requires QEMU)"
echo "To compile LibertyOS, for use in another VM software, or to run LibertyOS on real hardware, simply use the cargo build command."
echo "To test the kernel, use the cargo test command."
echo "If you encounter any issues, please open an issue on the main repository."
echo "https://github.com/LibertyOS-Development/kernel"
