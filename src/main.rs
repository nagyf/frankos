#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::vga_buffer::{set_fg_color, Color};

pub mod vga_buffer;

#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    set_fg_color(Color::Red);
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!!");
    loop {}
}

