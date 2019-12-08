// A "box" containing data.
// box can be either empty or it can
// hold data.

#[derive(Debug)]
enum DataBox <T>{
    Empty,
    Data(T),
}

use DataBox::*;

fn main() {
    let p = Data(10); // contains one integer
    let q = Data(1.2);
    let r = Data("hello");
    println!("{:?}, {:?}, {:?}", p, q, r);
}   
