#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use x86_64::instructions::hlt;

use crate::vga_buffer::{Color, set_fg_color};

mod vga_buffer;
mod interrupts;
mod gdt;

pub fn init() {
    interrupts::init_idt();
    gdt::init();
    unsafe {
        interrupts::PICS.lock().initialize();
    }

    x86_64::instructions::interrupts::enable();
}

#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    set_fg_color(Color::Red);
    println!("{}", info);

    hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    println!("It did not crash");

    hlt_loop();
}

fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
