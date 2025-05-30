use lazy_static::lazy_static;
use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::GlobalDescriptorTable;
use x86_64::structures::gdt::Descriptor;

pub const DOUBLE_FAULT_IST_INDEX: usize = 0;
lazy_static!(
    static ref TSS : TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8;STACK_SIZE] = [0;STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&raw const STACK);

            // Returns back the Stack end
            stack_start + STACK_SIZE
        };
        tss
    };
);

lazy_static!(
    static ref GDT: GlobalDescriptorTable = {
        let mut gdt = GlobalDescriptorTable::new();
        gdt.add_entry(Descriptor::kernel_code_segment());
        gdt.add_entry(Descriptor::tss_segment(&TSS));
        gdt
    };
);
