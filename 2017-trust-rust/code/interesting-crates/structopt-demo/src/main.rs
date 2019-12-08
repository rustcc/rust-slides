extern crate structopt;

#[macro_use]
extern crate structopt_derive;


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

}
