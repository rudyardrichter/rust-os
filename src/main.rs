#![no_main]
#![no_std]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::{hlt_loop, memory, println};
use x86_64::{structures::paging::Translate, VirtAddr};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    rust_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(physical_memory_offset) };
    let virt = VirtAddr::new(0xb8000);
    println!("{:?} -> {:?}", virt, mapper.translate_addr(virt));

    hlt_loop();
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}
