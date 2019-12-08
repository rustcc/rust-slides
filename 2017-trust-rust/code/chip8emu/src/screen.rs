// screen.rs

use std::collections::HashMap;

use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
//use sdl2::event::Event;
//use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

/// Default screen height in pixels
pub const SCREEN_HEIGHT:u16 = 32;

/// Default screen width in pixels
pub const SCREEN_WIDTH:u16 = 64;

pub const DEFAULT_SCALE_FACTOR: u32 = 5;

static WINDOW_TITLE: &'static str = "CHIP-8 Demo!";

/// CHIP-8 uses keys from 0,1,...9 and a, b, ... f.
/// These are assigned codes from 0, 1, ... 0xf.
lazy_static! {
    static ref KEYCODES:HashMap<u8, Keycode> = hashmap! {
         0x0 => Keycode::Num0,
         0x1 => Keycode::Num1,
         0x2 => Keycode::Num2,
         0x3 => Keycode::Num3,
         0x4 => Keycode::Num4,
         0x5 => Keycode::Num5,
         0x6 => Keycode::Num6,
         0x7 => Keycode::Num7,
         0x8 => Keycode::Num8,
         0x9 => Keycode::Num9,
         0xa => Keycode::A,
         0xb => Keycode::B,
         0xc => Keycode::C,
         0xd => Keycode::D,
         0xe => Keycode::E,
         0xf => Keycode::F,
    };
}

/// CHIP-8 uses only two colors: 0 for OFF and
/// 1 for ON.
lazy_static! {
    pub static ref PIXEL_COLORS:[Color; 2] = 
        [Color::RGBA(0, 0, 0, 255), 
         Color::RGBA(250, 250, 250, 255)];
}

pub struct Screen {
    scale_factor: u32,
    pub canvas: Canvas<Window>,
    pub events: EventPump,
    /// `mem' is a representation of the display within the
    /// virtual machine. If mem[i] is 1, the corresponding 
    /// pixel on the real screen is ON, otherwise OFF.
    ///  
    /// Had to use this because rust-sdl2 
    /// does not seem to provide an easy way to get the
    /// color of a pixel.
    mem: [u8; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
}

impl Screen {
    pub fn new(width: u32, height: u32, scale_factor: u32) -> Screen {
        let ctxt = sdl2::init().expect("SDL2 library initialization failed.");
        let video = ctxt.video().expect("Unable to get video subsystem.");
        let window = 
            video.window(WINDOW_TITLE, width * scale_factor, height * scale_factor)
            .position_centered()
            .opengl()
            .build()
            .expect("Unable to get Window");
        
        let mut canvas = window.into_canvas().build().expect("Unable to get canvas");

        canvas.set_draw_color(PIXEL_COLORS[0]);
        canvas.clear();
        canvas.present();
        
        let events = ctxt.event_pump().expect("Unable to get event pump"); 

        Screen{ 
            scale_factor: scale_factor,
            canvas: canvas, events: events,
            mem: [0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
        }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color:Color) {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(
            Rect::new(
                x as i32 * self.scale_factor as i32, 
                y as i32 * self.scale_factor as i32,
                self.scale_factor, self.scale_factor)).expect("Error in draw_point");
        let c = if color == PIXEL_COLORS[0] { 0 } else { 1 };
        self.mem[(y * u32::from(SCREEN_WIDTH) + x) as usize] = c;
    }

    /// Map an SDL Keycode to the numeric key value used
    /// by CHIP-8.
    fn keycode_to_keyval(k: Keycode) -> Option<u8> {
        for (chip8_val, sdl_code) in KEYCODES.iter() {
            if *sdl_code == k {
                return Some(*chip8_val);
            }
        }
        None
    }

    /// Do a blocking read from the keyboard. Return the
    /// associated CHIP-8 key value if a keypress is 
    /// detected and the pressed key is valid.
    pub fn read_key_blocking(&mut self) -> Option<u8> {
        loop {
            let e = self.events.wait_event();
            if let Event::KeyDown { keycode: Some(k), ..} = e {
                break Screen::keycode_to_keyval(k);
            }
        }
    }

    /// Do a non-blocking read from the keyboard. Return
    /// the associated CHIP-8 key value if a keypress is
    /// detected and the pressed key is valid.
    pub fn read_key_noblocking(&mut self) -> Option<u8> {
        if let Some(e) = self.events.poll_event() {
            match e {
                Event::KeyDown { keycode: Some(k), ..} => {
                        Screen::keycode_to_keyval(k)
                },
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> u8 {
        self.mem[(y * u32::from(SCREEN_WIDTH) + x) as usize]
    }
 }
