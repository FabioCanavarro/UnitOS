use crate::{gdt, pic::InterruptIndex};
use crate::pic::PICS;
use crate::{print, println};
use lazy_static::lazy_static;
use x86_64::instructions::port::{self, Port, PortGeneric, PortReadAccess, PortReadOnly, PortWriteAccess};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

/* NOTE: Initialize IDT as a static only when called
*        and create a mutable reference to its mutable static variable
*/

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt: InterruptDescriptorTable = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            // NOTE:
            // Switch stack pointer to DOUBLE_FAULT_IST_INDEX which is to another stack when double
            // fault happens
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX as u16);
        }
        idt[InterruptIndex::Timer.as_u8() as usize].set_handler_fn(timer_handler);
        idt[InterruptIndex::Keyboard.as_u8() as usize].set_handler_fn(keyboard_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
}

// Handler Funcs
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:?}", stack_frame)
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT\n{:#?}\n error_code: \t{:#?}",
        stack_frame, error_code
    );
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    // NOTE: THIS forms a deadlock as its asynchrous
    // It is never freed lol
    unsafe {
        // NOTE: Reason why this runs infinitely is that
        // when an interrupt happens, an EOI is sent which creates ends the current data sending by
        // the timer
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    let mut p: Port<u16> = Port::new(0x60);
    unsafe {

        let decodedkey = super::keyboard::process_key(p.read() as u8);
        match decodedkey.expect("Not a key lol").expect("None") {
            pc_keyboard::DecodedKey::RawKey(char) => println!("{:?}",char),
            pc_keyboard::DecodedKey::Unicode(char) => println!("{}",char)
        };
        PICS.lock()
                    .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}























