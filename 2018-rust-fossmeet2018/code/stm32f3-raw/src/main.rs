
// LD3, North LED, on PE9
// LD7, East LED, on PE11

#![no_std]
#![feature(used)]
#![feature(lang_items)]

extern crate cortex_m;
extern crate cortex_m_rt;

use cortex_m::asm;
use core::ptr;

fn main() {


    unsafe {
        const RCC_AHBENR: u32 = 0x40021000 + 0x14;
        const GPIOE_BSRR: u32 = 0x48001018;
        const GPIOE_MODER: u32 = 0x48001000;
 
        let x = ptr::read_volatile(RCC_AHBENR as *mut u32);
       
        ptr::write_volatile(RCC_AHBENR as *mut u32, x | (1 << 21));
       
        ptr::write_volatile(GPIOE_MODER as *mut u32, (1 << 18) | (1 << 22));    

        // Turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);

        // Turn on the "East" LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);

    }

    loop {}
}
// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C"  fn default_handler() {
    asm::bkpt();
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}

