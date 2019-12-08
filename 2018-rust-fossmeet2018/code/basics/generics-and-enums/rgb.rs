
// "enums" (sometimes called "sum-types") represent
// choice. A color is exactly one of either Red, Green or
// Blue.
enum Color {
    Red,
    Green,
    Blue,
}

use Color::*;


fn main() {
    let c = Red;

    // the match is "exhaustive", in the sense you
    // miss one possible value and the code will NOT
    // compile.
    match c {
        Red => println!("color red!"),
        Green => println!("color green!"),
       // Blue => println!("color blue!"),
    }
}


