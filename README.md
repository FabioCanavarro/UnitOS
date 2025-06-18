# UnitOS: Rust Bare-Bones OS

[![Rust](https://github.com/FabioCanavarro/UnitOS/actions/workflows/rust.yml/badge.svg)](https://github.com/FabioCanavarro/UnitOS/actions/workflows/rust.yml)

This is a simple, hobby operating system kernel written in Rust. It's designed for x86_64 architecture.

## Features

* **Bare-Bones Kernel:** A minimal kernel implementation.
* **VGA Text Output:** Writes text directly to the VGA buffer for display.
* **Intergrated Tests:** Be able to write intergrated tests or even unit tests.
* **Cpu Exception Handling** Be able to handle general errors.
* **Interupt Handling** Be able to handle cpu interrupts and hardware interrupts.
* **Keyboard Interaction** Be able to use the keyboard.

## Build and Run Instructions

Since this is a very basic OS, you'll need a way to compile it and run it in an environment like QEMU. Here's a general outline:

1.  **Prerequisites:**
    * Rust (nightly is recommended)
    * bootimage (as latest version is needed but the project uses an older version)
    * Qemu (Needed unless ran in a real hardware)

2.  **Clone the repository:**
    ```bash
    git clone https://github.com/FabioCanavarro/UnitOS
    cd UnitOS
    ```

3.  **Build the kernel:**
    ```bash
    rustup target add x86_64-unknown-none
    cargo build --target x86_64-unknown-none
    ```

4. **Run and build with cargo**
   ```bash
   cargo run
   ```

##  Further Development

Possible new features:
* **More drivers:** Add drivers for other hardware devices, such as the disk, and network interface or perhaps even the mouse
* **Memory management:** Implement a memory management system to allocate and deallocate memory
* **Basic input:** Implement mouse input
* **File system:** Design a simple file system
* **Multitasking:** Implement a basic scheduler to allow multiple programs to run concurrently
* **Screen Drawing:** Implement a class for drawing at the screen
* **Automatic Deployment:** Deploy to github automatically

