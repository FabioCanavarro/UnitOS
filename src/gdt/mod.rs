use lazy_static::lazy_static;
use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::Segment;
use x86_64::registers::segmentation::CS;
use x86_64::structures::gdt::SegmentSelector;
use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::GlobalDescriptorTable;
use x86_64::structures::gdt::Descriptor;

struct Selector {
    code_segment: SegmentSelector,
    tss_segment: SegmentSelector
}

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
    static ref GDT: (GlobalDescriptorTable, Selector) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_segment = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_segment = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selector{code_segment, tss_segment})
    };
);

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_segment);
        load_tss(GDT.1.tss_segment);
    }
}
