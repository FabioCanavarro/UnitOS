#![feature(abi_x86_interrupt)]
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;


static mut INTERRUPT_TABLE : InterruptDescriptorTable = InterruptDescriptorTable::new();

fn idt_init(){
    unsafe {
        INTERRUPT_TABLE.breakpoint.set_handler_fn(breakpoint);
        INTERRUPT_TABLE.load();
    }
}

// Handler Funcs
extern "x86-interrupt"
fn breakpoint_handler(stack_frame: InterruptStackFrame){
    println!("EXCEPTION: BREAKPOINT\n{:?}",stack_frame)
}
