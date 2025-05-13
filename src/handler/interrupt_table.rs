use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;


static mut IDT : InterruptDescriptorTable = InterruptDescriptorTable::new();

fn idt_init(){
    unsafe {
        #[allow(static_mut_refs)]
        IDT.breakpoint.set_handler_fn(breakpoint_handler);

        #[allow(static_mut_refs)]
        IDT.load();
    }
}

// Handler Funcs
extern "x86-interrupt"
fn breakpoint_handler(stack_frame: InterruptStackFrame){
    println!("EXCEPTION: BREAKPOINT\n{:?}",stack_frame)
}
