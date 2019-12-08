
// Put on LED on PC13 (blue pill board) 

#![no_std]
#![feature(used)]
#![feature(lang_items)]
#![feature(asm)]

extern crate cortex_m;
extern crate cortex_m_rt;

use cortex_m::asm;
use core::ptr;

#[inline(never)]
fn delay() {
    let count = 250_000;
    for _i in 0..count {
        unsafe{
            asm!("nop");
        }
    }
}


fn main() {


    unsafe {
        const RCC_APB2ENR: u32 = 0x4002_1000 + 0x18;

        const GPIOC_BSRR: u32 = 0x4001_1000 + 0x10;
        
        // GPIOC Config Register High
        const GPIOC_CRH: u32 = 0x4001_1000 + 0x4;
 
        let x = ptr::read_volatile(RCC_APB2ENR as *mut u32);
       
        // Enable PORT C
        ptr::write_volatile(RCC_APB2ENR as *mut u32, x | (1 << 4));
        
        let x = ptr::read_volatile(GPIOC_CRH as *mut u32);

        let conf = x & !(0xf_u32 << 20);
    
        let conf = conf | (0b0010_u32 << 20);

        // Configure PC13 as Push-Pull Output
        ptr::write_volatile(GPIOC_CRH as *mut u32, conf);

        loop {
            // Put ON PC13
            ptr::write_volatile(GPIOC_BSRR as *mut u32, 1 << (16 + 13));

            delay();
            
            // Put OFF PC13
            ptr::write_volatile(GPIOC_BSRR as *mut u32, 1 <<  13);

            delay();
        }
        
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

