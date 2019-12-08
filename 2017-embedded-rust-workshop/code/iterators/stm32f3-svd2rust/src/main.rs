
// LD3, North LED, on PE9
// LD7, East LED, on PE11

// Compile in Debug mode only

#![no_std]
#![feature(used)]
#![feature(lang_items)]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f30x;

use cortex_m::asm;
use stm32f30x::{Peripherals, GPIOE};

fn delay(mut x: u32) {
    while x > 0 {
        x -= 1;
    }
}

struct Lfsr {
    start: u16,
}

impl Iterator for Lfsr {
    type Item = u16;
    
    fn next(&mut self) -> Option<Self::Item> {
        let bit  = ((self.start >> 0) ^ 
                    (self.start >> 2) ^ 
                    (self.start >> 3) ^ 
                    (self.start >> 5)) & 1;

        self.start =  (self.start >> 1) | (bit << 15);
        Some(bit)
    }
}

fn new_lfsr(n: u16) -> Lfsr {
    Lfsr { start: n }
}

fn pe9_on(gpioe: &mut GPIOE) {
    gpioe.bsrr.write(|w| unsafe { w.bits(1 << 9) });
}

fn pe9_off(gpioe: &mut GPIOE) {
    gpioe.bsrr.write(|w| unsafe { w.bits(1 << (16 + 9)) });
}

fn main() {

    let p = Peripherals::take().unwrap();   

    let mut gpioe = p.GPIOE;
    let rcc = p.RCC;
 
    rcc.ahbenr.modify(|r, w| unsafe { w.bits(r.bits() | (1 << 21)) });  
    gpioe.moder.write(|w| unsafe { w.bits(1  << 18) });

    let l = new_lfsr(0x1234);
    for bit in l {
        if bit == 0 {
            pe9_off(&mut gpioe);
        } else {
            pe9_on(&mut gpioe);
        }
        delay(10000);
    }
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

