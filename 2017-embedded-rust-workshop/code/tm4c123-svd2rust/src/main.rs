// LED's on PF1, PF2, PF3

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate tm4c123x;

const PIN_RED_LED: u32 = 1;

use cortex_m::asm;

fn main() {
    let p = tm4c123x::Peripherals::take().unwrap();

    let gpiof = p.GPIO_PORTF;
    let sysctl = p.SYSCTL;

    // Enable PORTF
    sysctl.rcgcgpio.modify(|_, w| w.r5().bit(true));
    // Need to wait for sometime after modifying rcgcgpio
    let _t = sysctl.rcgcgpio.read(); 
    
    // Enable digital IO on the specified pin
    gpiof.den.modify(|r, w| unsafe { w.bits(r.bits() | (1 << PIN_RED_LED)) });
    // Make it an output pin
    gpiof.dir.modify(|r, w| unsafe { w.bits(r.bits() | (1 << PIN_RED_LED)) });
    
    // Write a 1 to the pin - RED LED lights up!
    gpiof.data.modify(|r, w| unsafe { w.bits(r.bits() | (1 << PIN_RED_LED))  });
    
    loop {
    }

}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
