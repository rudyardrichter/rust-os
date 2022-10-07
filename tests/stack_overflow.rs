#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use rust_os::{exit_qemu, hlt_loop, serial_print, serial_println, QemuExitCode};
use volatile::Volatile;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[allow(unreachable_code)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");
    rust_os::gdt::init();
    init_test_idt();
    stack_overflow();
    panic!("Execution continued")
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[allow(unconditional_recursion, unreachable_code)]
fn stack_overflow() -> ! {
    stack_overflow();
    // prevent tail recursion optimization, making sure additional stack frames are created
    Volatile::new(0).read();
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(rust_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    hlt_loop();
}
