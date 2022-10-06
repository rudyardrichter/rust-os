#![feature(custom_test_frameworks)]
#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![test_runner(rust_os::test_runner)]

use core::panic::PanicInfo;
use rust_os::{exit_qemu, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

// In this test run we will expext panics, therefore exit with success code from the panic
// handler.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failure);
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn should_fail() {
    assert_eq!(0, 1);
}
