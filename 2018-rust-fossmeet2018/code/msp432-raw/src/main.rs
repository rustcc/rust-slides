
// LED's on P2.0, P2.1, P2.2
// P1 and P2 together forms PA

#![no_std]
#![feature(used)]
#![feature(lang_items)]

extern crate cortex_m;
extern crate cortex_m_rt;

use cortex_m::asm;
use core::ptr::{read_volatile, write_volatile};

fn main() {


    unsafe {
        const PADIR: u32 = 0x4000_4c00 + 0x4;
        const PAOUT: u32 = 0x4000_4c00 + 0x2;
    
        // P1 is Lower 8 bits of PA and P2 is higher
        // 8 bits.
        write_volatile(PADIR as *mut u16, 1 << 8);
        write_volatile(PAOUT as *mut u16, 1 << 8);
        
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

