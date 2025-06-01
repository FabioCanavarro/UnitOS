#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![feature(abi_x86_interrupt)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use lazy_static::lazy_static;
use rusty_os::{exit_qemu, gdt, halt, serial_print, serial_println, test_panic_handler};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow....\t");

    gdt::init();

    init_test_idt();

    stack_overflow();

    panic!("Continued to run this line, even after stack has overflowed");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
    halt()
}

lazy_static! (
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX as u16);
        }
        idt
    };
);

pub extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(rusty_os::QemuExitCode::Success);
    halt()
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
    volatile::Volatile::new(0).read(); // prevent tail reduction optimization lol
}
