#![feature(abi_x86_interrupt)]
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;


static mut INTERRUPT_TABLE : InterruptDescriptorTable = InterruptDescriptorTable::new();

extern "x86-interrupt"
fn breakpoint(stack_frame: InterruptStackFrame){
    println!("EXCEPTION: BREAKPOINT\n{:?}",stack_frame)
}
