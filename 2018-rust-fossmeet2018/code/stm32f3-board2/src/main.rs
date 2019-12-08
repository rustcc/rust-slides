//! Turns all the user LEDs on
#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_std]
#![feature(used)]
#![feature(lang_items)]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate f3;

use f3::hal::stm32f30x;
use f3::hal::prelude::*;
use f3::hal::delay::Delay;

use f3::led::Leds;

use cortex_m::asm;

fn main() {
    let p = stm32f30x::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
 
    let mut flash = p.FLASH.constrain();   
    let mut rcc = p.RCC.constrain();
    let gpioe = p.GPIOE.split(&mut rcc.ahb);
    let clocks = rcc.cfgr.freeze(&mut flash.acr);    
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut leds = Leds::new(gpioe);
    
    loop {
        leds.iter_mut().for_each(|led| {
                            led.on();   
                            delay.delay_ms(100u8);
                        });
        leds.iter_mut().for_each(|led| {
                            led.off();
                            delay.delay_ms(100u8);
                        });
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

