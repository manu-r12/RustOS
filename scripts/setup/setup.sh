#!/bin/bash

echo "Installing Rust nightly..."
rustup install nightly
rustup default nightly

echo "Adding required Rust components..."
rustup target add x86_64-blog_os --toolchain nightly
rustup target add x86_64-unknown-none
rustup component add rust-src --toolchain nightly
rustup component add llvm-tools-preview

echo "Installing bootimage..."
cargo install bootimage

echo "Installing QEMU..."
brew install qemu

echo "Setup complete! 🎉"
