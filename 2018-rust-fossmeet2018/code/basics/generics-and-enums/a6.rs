// A "box" containing data.
// box can be either empty or it can
// hold data.

#[derive(Debug)]
enum DataBox {
    Empty,
    Data(i32),
}

fn main() {
    let p = DataBox::Empty; // empty box
    let q = DataBox::Data(10); // contains one integer

    println!("{:?}, {:?}", p, q);
}   
