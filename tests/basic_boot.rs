#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rusty_os::{halt, println, test_panic_handler};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    halt()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
    halt()
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
