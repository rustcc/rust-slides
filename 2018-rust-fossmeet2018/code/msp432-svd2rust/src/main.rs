#![feature(used)]
#![no_std]

// RGB LED on P2.0, P2.1, P2.2
// P1 & P2 grouped together as PA

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate msp432p401r;


use cortex_m::asm;

fn main() {

    let p = msp432p401r::Peripherals::take().unwrap();

    let dio = p.DIO;

    dio.padir.modify(|r, w| unsafe { w.p2dir().bits(r.p2dir().bits() | 2) });

    dio.paout.modify(|r, w| unsafe { w.p2out().bits(r.p2out().bits() | 2) });

    loop {}

}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
