
// LD3, North LED, on PE9
// LD7, East LED, on PE11

#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_std]
#![feature(used)]
#![feature(lang_items)]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f30x_hal as hal;

use hal::stm32f30x;
use hal::prelude::*;

use hal::gpio::{Input, Floating, Output, PushPull};

use hal::gpio::gpioe::{PE9, PE11, PE8};

use cortex_m::asm;

fn main() {

    let p = stm32f30x::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let mut gpioe = p.GPIOE.split(&mut rcc.ahb);

    let mut pe9: PE9<Output<PushPull>> = gpioe.pe9.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    let mut pe11: PE11<Output<PushPull>> = gpioe.pe11.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        

    pe9.set_high();
    pe11.set_high();
 
    //let mut pe8: PE8<Input<Floating>> = gpioe.pe8;
    //pe8.set_high();
   // let mut pe8_1: PE8<Output<PushPull>> = gpioe.pe8.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

   // let mut pe9_1 = pe9.into_floating_input(&mut gpioe.moder, &mut gpioe.pupdr);
   // pe9.set_high();   
    
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

