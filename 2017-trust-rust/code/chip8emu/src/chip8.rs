// chip8.rs

use screen;
use cpu;
use std::{thread, time};

const NTICKS: u32 = 8;
const DELAY_MS: u64 = 1;


pub fn chip8_run(font_file: &str, game_file: &str, scale_factor: u32) {
    let  s = screen::Screen::new(
        u32::from(screen::SCREEN_WIDTH), 
        u32::from(screen::SCREEN_HEIGHT), 
        scale_factor);

    let delay = time::Duration::from_millis(DELAY_MS); 
    let mut tick_counter = 0;

    let mut c = cpu::CPU::new(Some(s));
    c.load_rom(font_file, 0);
    c.load_rom(game_file, cpu::PC_START);

    loop {
        thread::sleep(delay);  
        if tick_counter == NTICKS { 
            c.decrement_counters();
            tick_counter = 0;
        }     
        c.execute_insn();
        tick_counter += 1;
    }
 
}