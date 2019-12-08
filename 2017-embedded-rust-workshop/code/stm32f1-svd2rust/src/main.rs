
// Blink PC13 LED on the blue pill board

#![no_std]
#![feature(used)]
#![feature(lang_items)]
#![feature(asm)]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f103xx;

use cortex_m::asm;
use stm32f103xx::{Peripherals};

#[inline(never)]
fn delay() {
    for _i in 1..250_000 {
        unsafe {
            asm!("nop");
        }
    }
}

fn main() {

    let p = Peripherals::take().unwrap();

    let gpioc = p.GPIOC;
    let rcc = p.RCC;
    
    // Enable PC13
    rcc.apb2enr.modify(|_, w| w.iopcen().set_bit());
    
    // PC13 configured as push/pull output, 2mhz.
    gpioc.crh.modify(|_, w| w.mode13().output2().cnf13().push());
 
    loop {
        // PC13 LED On (note: LED on when GPIO off)
        gpioc.bsrr.write(|w| w.br13().reset());
        delay();

        // PC13 LED Off 
        gpioc.bsrr.write(|w| w.bs13().set());
        delay();
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

