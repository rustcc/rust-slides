use clap::{App, Arg};

use std::fs;

use webp::png_encode_webp;

fn main() {
    let matches = App::new("cwebp")
        .author("hawkingrei <hawkingrei@gmail.com>")
        .arg(
            Arg::with_name("o")
                .long("o")
                .takes_value(true)
                .help("output"),
        )
        .arg(
            Arg::with_name("i")
                .long("i")
                .takes_value(true)
                .help("input"),
        )
        .get_matches();


    let output = matches.value_of("o").unwrap_or("out.webp");
    let input = matches.value_of("i").unwrap();

    let data = fs::read(input).unwrap();
    let result = png_encode_webp(&data.clone()).unwrap();
    fs::write(output, result).unwrap();
       
}
