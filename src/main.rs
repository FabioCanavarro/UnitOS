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

    rusty_os::init();

    // NOTE: Breakpoint exception
    interrupts::int3();

    println!("here");

    /*
        unsafe {
            // Fails here because accessing invalid address
            *(0x10000 as *mut u8) = 23;

            
            // NOTE: set a memory adress at the most recent stack to 0x10000 or 65536 then get its reference
            // "&" and then reads it using "*"
            //
            // println!("{:?}",*(&0x10000));
            
            // Reads at adress 0x10000
            let l: *const u8 = 0x10000 as *const u8;
            println!("{:?}",*l);
        };
    */
    // NOTE: Triggering a Page fault error, that has no handler function
    /*
        unsafe {
            // accessing at 0xFFFFFFFFFFFFFFF which is invalid, cuz way too high
             *(0xFFFFFFFFFFFFFFF as *mut u8) = 42;

            // NOTE:  Causes inifinite device restarting since there is no double fault handler func,
            // which causes triple fault restarting
        }
    */

    /*
        fn stack_overflow() {
            stack_overflow(); // for each recursion, the return address is pushed
        }

        // trigger a stack overflow
        stack_overflow();
        // NOTE: Causes a triple fault, cuz double fault need to push to stack, and stack already overflowed
        // which causes another exception
    */
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
