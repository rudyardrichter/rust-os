#![no_main]
#![no_std]

use core::panic::PanicInfo;
use rust_os::{hlt_loop, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    rust_os::init();

    // Demonstrate page fault
    let ptr = 0xdeadbeef as *mut u32;
    unsafe {
        *ptr = 42;
    }

    hlt_loop();
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}
