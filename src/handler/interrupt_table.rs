use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;


static mut IDT : InterruptDescriptorTable = InterruptDescriptorTable::new();

fn idt_init(){
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint);
        IDT.load();
    }
}

// Handler Funcs
#[feature(abi_x86_interrupt)]
extern "x86-interrupt"
fn breakpoint_handler(stack_frame: InterruptStackFrame){
    println!("EXCEPTION: BREAKPOINT\n{:?}",stack_frame)
}
