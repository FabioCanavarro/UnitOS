# Rust Bare-Bones OS

This is a simple, hobby operating system kernel written in Rust. It's designed for x86_64 architecture and uses a no_std environment, meaning it doesn't rely on the standard Rust library. This project is primarily for learning purposes, exploring OS development concepts, and experimenting with Rust in a low-level environment.

## Features

* **Bare-Bones Kernel:** A minimal kernel implementation.
* **No Standard Library:** Uses `#![no_std]` to avoid dependencies on the standard Rust library.
* **VGA Text Output:** Writes text directly to the VGA buffer for display.
* **Panic Handling:** Custom panic handler to print error messages.
* **Basic Printing:** Includes a `println!` macro for basic output.

## Build and Run Instructions

Since this is a very basic OS, you'll need a way to compile it and run it in an environment like QEMU.  Here's a general outline:

1.  **Prerequisites:**
    * Rust (nightly is recommended)
    * bootimage (as current version is needed and the project uses an older version)
    * Qemu (Needed unless ran in a real hardware)

2.  **Clone the repository:**
    ```bash
    git clone <your_repository_url>
    cd <your_repository_name>
    ```

3.  **Build the kernel:**
    ```bash
    rustup target add x86_64-unknown-none
    cargo build --target x86_64-unknown-none
    ```

4.  **Run with QEMU**
    ```bash
    qemu-system-x86_64 -kernel target/x86_64-unknown-none/debug/your_kernel_name
    ```

or 

3. **Run and build with cargo**
   ```bash
   cargo run
   ```
##  Further Development

This is a very basic starting point.  Here are some ideas for expanding this kernel:

* **More drivers:** Add drivers for other hardware devices, such as the keyboard, disk, and network interface.
* **Memory management:** Implement a memory management system to allocate and deallocate memory.
* **Interrupt handling:** Set up interrupt handlers to respond to hardware interrupts.
* **Basic input:** Implement keyboard input.
* **File system:** Design a simple file system.
* **Multitasking:** Implement a basic scheduler to allow multiple programs to run concurrently.

##  Disclaimer

This project is for educational purposes only.  It's not intended for production use.
