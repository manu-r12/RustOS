# 🛠️ Rust OS Project Setup Guide

Welcome to the RustOS project!

This guide will walk you through setting up everything you need — and solving any issues you might face — while building your own operating system based on Philipp Oppermann’s [blog_os](https://os.phil-opp.com/).

---

## 📦 1. Install Required Tools

You need:

- **Rust nightly** toolchain
- **Rust-src** (standard library source code)
- **LLVM tools preview**
- **Bootimage** (to create bootable images)
- **QEMU** (to emulate the OS)

Install everything with:

```bash
# Install Rust Nightly
rustup install nightly
rustup default nightly

# Add components
rustup component add rust-src --toolchain nightly
rustup component add llvm-tools-preview --toolchain nightly

# Add targets
rustup target add x86_64-blog_os --toolchain nightly
rustup target add x86_64-unknown-none

# Install bootimage
cargo install bootimage

# Install QEMU (for Mac users)
brew install qemu
🏗️ 2. Configure Rust Toolchain
Create a file named rust-toolchain.toml in the project root:

toml
Copy
Edit
[toolchain]
channel = "nightly"
targets = ["x86_64-blog_os"]
This forces everyone to use the correct toolchain and targets.

🔥 3. Common Issues and Solutions
Here’s a big list of common issues you might face during the whole blog journey:


Problem	Cause	Solution
Cargo.lock does not exist when building	Rust source missing	Run rustup component add rust-src --toolchain nightly
cannot find target 'x86_64-blog_os'	Custom target not added	Make sure you have a file like x86_64-blog_os.json, and add it properly during build: cargo build --target x86_64-blog_os.json
cargo bootimage fails	Bootimage not installed	Install it: cargo install bootimage
error: target may not be used until the rust-src component is installed	Rust-src missing	Run rustup component add rust-src --toolchain nightly
No such file or directory: bootimage	cargo-bootimage not in PATH	Ensure ~/.cargo/bin is in your PATH, or run source $HOME/.cargo/env
qemu-system-x86_64: command not found	QEMU not installed	Install via Homebrew: brew install qemu
Kernel panic during QEMU run	Mistake in Rust code (bad memory access, wrong pointer)	Double-check unsafe blocks. Follow the blog carefully. Start debugging with serial output.
can't compile: missing alloc crate / core crate	Missing no_std setup	Check Cargo.toml, add #![no_std] to your code, and ensure you depend on core/alloc manually if needed.
linker errors when building	Linker settings missing	Make sure you have a .cargo/config.toml with correct linker config. Example below 👇
No target JSON file found	Missing x86_64-blog_os.json	Copy it from the blog or create it based on this template
🔧 4. Required .cargo/config.toml
Inside your project, create .cargo/config.toml:

toml
Copy
Edit
[build]
target = "x86_64-blog_os.json"

[target.x86_64-blog_os]
runner = "bootimage runner"

[unstable]
build-std = ["core", "compiler_builtins", "alloc"]
build-std-features = ["compiler-builtins-mem"]
This ensures:

Cargo knows which target you are using

Automatically builds required parts of standard libraries (core, alloc)

🚀 5. Commands to Build, Boot, and Test

Step	Command
Build the kernel	cargo build --target x86_64-blog_os.json
Create bootable image	cargo bootimage
Run on QEMU	qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-your_project_name.bin
```
