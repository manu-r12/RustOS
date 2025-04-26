#  Rust OS 
<p align="center">
  <img src="https://github.com/user-attachments/assets/4d5451a8-a2a4-4473-b749-3182121ca05a" alt="Rust OS Logo" width="200"/>
</p>


## Team Members

- Manu – Systems Developer
- Ayush – Memory & Process Management 
- Manish – Bootloader & Architecture
- Adnan – Debugging & Optimization

---

## Resources We’re Using

- Philipp Oppermann’s [Writing an OS in Rust](https://os.phil-opp.com/)
- Rust OSDev GitHub
- Rust Official Documentation

---

## Inspiration

This project is fueled by our curiosity to deeply understand how systems work under the hood.  
We aim not only to **build** but to **truly understand** every piece of the stack — from booting the hardware to managing processes and memory.

---

##  About the Project

**Rust OS** is a fully handcrafted operating system project, built completely from scratch using the [Rust programming language](https://www.rust-lang.org/).  
We have successfully completed all major components of the blog_os tutorial and are now taking the next steps: **extending the OS** with advanced features like a **basic graphical interface** and a **simple filesystem**.

Our journey so far:
- Bootstrapped with a custom bootloader
- Set up memory management (heap, paging)
- Implemented interrupt handling and basic multitasking
- Built dynamic memory allocation without a standard library
- Added process/task switching and timer interrupts

###  Current Focus
We are now extending the operating system with:
- 🖥️ **Basic GUI:** Framebuffer graphics, simple windows, mouse cursor
- 📂 **Simple Filesystem:** In-memory filesystem with basic file operations

---

##  Tech Stack

- **Language:** Rust
- **Bootloader:** [bootimage](https://github.com/rust-osdev/bootimage)
- **Tooling:** QEMU for virtualization, Cargo for building
- **Architecture:** x86_64
- **Framework:** no_std environment

---

## How to Build and Run

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
