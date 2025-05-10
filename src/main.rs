#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod serial;
pub mod vga;

#[cfg(test)]
mod tests;

use tests::Tests;
use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    /*
        * NOTE: From my understanding
        * the index starts from 0 so
        * the 0xb8000 should be the first u8 of Hello
        * 0 * 2 + 1 = 1, to put the color code infront
        * then continue on from there

        for (i,&byte) in HELLO.iter().enumerate(){
            let step: isize = i as isize * 2;
            unsafe {
                *vga_buffer.offset(step) = byte;
                *vga_buffer.offset(step + 1) = 0xb;

            };
        }
    */

    println!("UnitOs");

    // Running all tests
    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
enum QemuExitCode {
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

fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
