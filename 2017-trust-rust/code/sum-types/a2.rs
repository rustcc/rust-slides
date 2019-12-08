
#[derive(Debug)]
enum Shape {
    Circle(u32),
    Square (u32),
    Rectangle {ht: u32, wid: u32},
}

fn main() {
    let s1 = Shape::Circle(10);
    let s2 = Shape::Square(5);
    let s3 = Shape::Rectangle {ht: 10, wid: 2};

    println!("{:?}", s3);
}

     
