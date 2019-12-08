//! Controls an LED on PC13

#![no_std]
#![feature(lang_items)]
#![feature(used)]
#![feature(asm)]

extern crate cortex_m;
extern crate stm32f103xx_hal as hal;

use hal::prelude::*;
use hal::stm32f103xx;
use cortex_m::asm;

#[inline(never)]
fn delay() {
    for _i in 0..250_000 {
        unsafe {
            asm!("nop");
        }
    }
}

fn main() {
    let p = stm32f103xx::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();


    let mut gpioc = p.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    
    loop {   
        led.set_high();
        delay();
        led.set_low();
        delay();
    }
}


#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}


