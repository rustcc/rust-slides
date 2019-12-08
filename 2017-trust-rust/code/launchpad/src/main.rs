
// Do not do a release build if you want to see
// flashing LED's! The "delay" function in the "led"
// module will get optimized away during a release
// build.

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate volatile_register;

mod led;

use cortex_m::asm;
use core::iter;
use led::{portf_init, red_led, green_led, blue_led};

fn main() {
    portf_init();
    let leds = [red_led(), green_led(), blue_led()];

    loop {
        leds.iter()
            .zip(leds.iter().skip(1).chain(iter::once(&leds[0])))
            .for_each(|(current, next)| {
                next.on();
                current.off();
                led::delay(500000);
             });
    }
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
