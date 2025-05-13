pub mod interrupt_table;

// Tests

#[test_case]
fn test_breakpoint_exception() {
    // Invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}

