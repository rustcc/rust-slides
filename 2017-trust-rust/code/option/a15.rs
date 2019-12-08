
use std::fs::File;


fn main() {
    let mut f = File::open("foo.txt");

    println!("{:?}", f);
}
