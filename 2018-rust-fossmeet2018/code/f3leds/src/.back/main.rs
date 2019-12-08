#![feature(lang_items)]
#![no_main]
#![no_std]
#![feature(used)]
extern crate f3;
extern crate cortex_m;
extern crate cortex_m_rt;
use cortex_m::asm;

#[export_name = "main"]
pub fn main() -> ! {
    loop {}
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[export_name = "_default_exception_handler"]
pub extern "C" fn handler() {
    loop {}
}

#[export_name = "_init"]
pub fn init() {}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}

