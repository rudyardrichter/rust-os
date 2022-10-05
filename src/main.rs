#![cfg_attr(not(test), no_std)]
#![no_main]

extern crate lazy_static;
extern crate spin;
extern crate volatile;

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");

    panic!("the disco");
}
