
// LD3, North LED, on PE9
// LD7, East LED, on PE11

#![no_std]
#![feature(used)]
#![feature(lang_items)]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f30x;

use cortex_m::asm;
use stm32f30x::{Peripherals, GPIOE};

fn main() {

    let p = Peripherals::take().unwrap();   

    let gpioe = p.GPIOE;
    let rcc = p.RCC;
 

    rcc.ahbenr.modify(|r, w| w.iopeen().set_bit());    

    gpioe.moder.write(|w| w.moder11().output().moder9().output());
    
    gpioe.bsrr.write(|w| w.bs9().set().bs11().set());


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

