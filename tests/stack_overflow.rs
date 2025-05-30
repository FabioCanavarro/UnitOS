#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;

use lazy_static::lazy_static;
use rusty_os::{gdt, serial_println, test_panic_handler};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    gdt::init();
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
    loop {}
}

fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
    volatile::Volatile::new(0).read(); // prevent tail reduction optimization lol
}

#[test_case]
#[allow(unconditional_recursion)]
fn stack_overflow_test() {
    serial_println!("stack_overflow::stack_overflow...\t");
    stack_overflow();
    panic!("Ran succesfully without crashing");
}

