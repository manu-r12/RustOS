# 🔧 Rust OS – A Minimal Operating System in Rust

Welcome to **Rust OS**, a hobby operating system built from scratch using the [Rust programming language](https://www.rust-lang.org/). This project is our deep dive into systems programming, memory management, and low-level architecture.

## 🚀 About the Project

This is a fun and educational project where we’re building a simple operating system to understand how computers work at the lowest levels. From bootloaders to kernels, we're doing it all ourselves — the Rust way!

Our goals:
- Learn low-level programming concepts
- Understand how operating systems manage memory, processes, and hardware
- Build a working minimal OS using Rust, without standard libraries

## 🧠 Tech Stack

- **Language:** Rust
- **Bootloader:** [bootimage](https://github.com/rust-osdev/bootimage)
- **Tooling:** QEMU for emulation, Cargo for building
- **Architecture:** x86_64

## 🛠️ How to Build and Run

> Prerequisites: Rust nightly, QEMU, bootimage

```bash
# Install Rust Nightly
rustup override set nightly
rustup component add rust-src

# Install bootimage
cargo install bootimage

# Build the OS
cargo bootimage

# Run it using QEMU
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-rust-os.bin
