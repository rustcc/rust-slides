// A "box" containing data.
// box can be either empty or it can
// hold data.

#[derive(Debug)]
enum DataBox {
    Empty,
    Data(i32),
}

use DataBox::*;

fn main() {
    let p = Empty; // empty box
    let q = Data(10); // contains one integer

    println!("{:?}, {:?}", p, q);
}   
