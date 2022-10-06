use crate::gdt;
use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

lazy_static! {
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
        let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
        let stack_end = stack_start + STACK_SIZE;
        tss.interrupt_stack_table[gdt::DOUBLE_FAULT_IST_INDEX as usize] = stack_end;
        tss
    };
}

pub fn init_idt() -> () {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) -> () {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    panic!(
        "EXCEPTION: DOUBLE FAULT ({})\n{:#?}",
        error_code, stack_frame
    );
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
