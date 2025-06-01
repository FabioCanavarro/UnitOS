#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use rusty_os::test_trait::Tests;
use rusty_os::{QemuExitCode, exit_qemu, println, test_runner};
use x86_64::instructions::interrupts;
use x86_64::instructions::port::Port;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("UnitOs");

    rusty_os::init();
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
