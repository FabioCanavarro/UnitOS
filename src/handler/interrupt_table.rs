use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

/* NOTE: Initialize IDT as a static only when called
*        and create a mutable reference to its mutable static variable
*/
lazy_static!(
    static ref IDT: InterruptDescriptorTable = {
        let mut idt: InterruptDescriptorTable = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt 
    };
);

pub fn init_idt(){
    IDT.load();
}

// Handler Funcs
extern "x86-interrupt"
fn breakpoint_handler(stack_frame: InterruptStackFrame){
    println!("EXCEPTION: BREAKPOINT\n{:?}",stack_frame)
}

