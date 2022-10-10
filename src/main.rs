#![no_main]
#![no_std]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::{hlt_loop, memory, println};
use x86_64::structures::paging::{FrameAllocator, PhysFrame};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    rust_os::init();

    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    let mut frame: PhysFrame;
    for i in 1..=5 {
        frame = frame_allocator.allocate_frame().expect("no frame");
        println!("Got frame {}: {:?}", i, frame);
    }

    hlt_loop();
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}
