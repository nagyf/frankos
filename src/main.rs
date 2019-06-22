#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod vga_buffer;

#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!!");
    panic!("hahaha this is the end");
    loop {}
}

