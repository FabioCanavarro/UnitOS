#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rusty_os::{
    halt, println, test_panic_handler,
    vga::{BUFFER_HEIGHT, WRITER},
};
use x86_64::instructions::interrupts;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    halt()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
    loop {}
    halt()
}

#[test_case]
fn test_println_output() {
    let s = "Testing!!!";
    println!("{}", s);
    // WARN: THIS IS A TEMPORARY FIXED, TO DISABLE INTERRUPTS
    interrupts::without_interrupts(|| {
        for (i, c) in s.chars().enumerate() {
            let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT as usize - 2][i].read();
            assert_eq!(char::from(screen_char.char), c);
        }
    });
}

#[test_case]
fn test_multi_line_print() {
    let s = "Testing";
    let s2 = "Testing2nd!!!";

    println!("{}", s);
    println!("{}", s2);

    // 1st line

    // WARN: THIS IS A TEMPORARY FIXED, TO DISABLE INTERRUPTS
    interrupts::without_interrupts(|| {
        for (i, c) in s.chars().enumerate() {
            let char = WRITER.lock().buffer.chars[BUFFER_HEIGHT as usize - 3][i].read();
            assert_eq!(char::from(char.char), c);
        }
    });

    // 2nd line

    // WARN: THIS IS A TEMPORARY FIXED, TO DISABLE INTERRUPTS
    interrupts::without_interrupts(|| {
        for (i, c) in s2.chars().enumerate() {
            let char = WRITER.lock().buffer.chars[BUFFER_HEIGHT as usize - 2][i].read();
            assert_eq!(char::from(char.char), c);
        }
    });
}
