#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused_imports)]

pub mod pic;
pub mod gdt;
pub mod handler;
pub mod serial;
pub mod test_trait;
pub mod vga;

use core::panic::PanicInfo;
use test_trait::Tests;
use x86_64::instructions::{interrupts, port::Port};

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();

    #[cfg(test)]
    test_main();

    loop {}
}

pub fn test_panic_handler(info: &PanicInfo) {
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
}

#[cfg(test)]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
    loop {}
}

pub fn test_runner(tests: &[&dyn Tests]) {
    serial_println!("Running {} tests\n", tests.len());
    for test in tests {
        test.run();
    }
    serial_print!("\n");
    serial_println!("Exit Code: 1");
    serial_println!("Success\n");

    exit_qemu(QemuExitCode::Success);
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum QemuExitCode {
    Success,
    Failed,
}

impl From<QemuExitCode> for u32 {
    fn from(value: QemuExitCode) -> Self {
        match value {
            QemuExitCode::Success => 0x10,
            QemuExitCode::Failed => 0x11,
        }
    }
}

#[allow(dead_code)]
pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn init() {
    handler::interrupt_table::init();
    gdt::init();
    unsafe {
        pic::PICS.lock().initialize();
    }
    interrupts::enable();
}
