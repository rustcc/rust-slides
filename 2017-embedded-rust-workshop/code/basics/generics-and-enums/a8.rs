// A "box" containing data.
// box can be either empty or it can
// hold data.

#[derive(Debug)]
enum DataBox1 {
    Empty,
    Data(i32),
}

#[derive(Debug)]
enum DataBox2 {
    Empty,
    Data(f32),
}


fn main() {
    let p = DataBox1::Empty;
    let q = DataBox2::Data(1.2);

    println!("{:?}, {:?}", p, q);
}   
