
// LED's on PF1, PF2, PF3

#![no_std]
#![feature(used)]
#![feature(lang_items)]

extern crate cortex_m;
extern crate cortex_m_rt;

use cortex_m::asm;
use core::ptr::{read_volatile, write_volatile};

fn main() {


    unsafe {
        const SYSCTL_RCGCGPIO: u32 = 0x400f_e000 + 0x608; 
        const GPIOF_DEN: u32 = 0x4002_5000 + 0x51c;
        const GPIOF_DIR: u32 = 0x4002_5000 + 0x400;
        const GPIOF_DATA: u32 = 0x4002_5000 + 0x3fc;
    
        let r = read_volatile(SYSCTL_RCGCGPIO as *mut u32);
        write_volatile(SYSCTL_RCGCGPIO as *mut u32, r | 0x20);
        let _x = read_volatile(SYSCTL_RCGCGPIO as *mut u32);

        // Put on BLUE LED. (on PF2)
        write_volatile(GPIOF_DEN as *mut u32, 0x4);
        write_volatile(GPIOF_DIR as *mut u32, 0x4);
        write_volatile(GPIOF_DATA as *mut u32, 0x4);
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

