

mod cpu;
mod screen;
mod chip8;

extern crate rand;
extern crate sdl2;
extern crate structopt;

#[macro_use]
extern crate structopt_derive;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long = "fontfile", help = "Name of the file containing fonts")]
    font_file: String,
    #[structopt(long = "gamefile", help = "Name of the file containing game code")]
    game_file: String,
    #[structopt(long = "scale", help = "The scale factor of the Window. Default is 5")]
    scale_factor: Option<u32>,
}



fn main() {
    let opt = Opt::from_args();
    let mut scale_factor: u32 = screen::DEFAULT_SCALE_FACTOR; 

    if let Some(s) = opt.scale_factor {
        scale_factor = s;
    }

    chip8::chip8_run(&opt.font_file, &opt.game_file, scale_factor);

}
