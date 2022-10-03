#![cfg_attr(not(test), no_std)]
#![no_main]

extern crate volatile;

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*
    for (i, byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = *byte;
            *vga_buffer.offset(i as isize * 2 + 1) = COLOR;
        }
    }
    */
    vga_buffer::print_something();

    loop {}
}
