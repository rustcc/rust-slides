
//Add an "ellipse" to the code
//Area of ellipse = 
//pi*semi_major_axis*semi_minor_axis

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Square (f64),
    Rectangle {ht: f64, wid: f64},
}

fn area(s: Shape) -> f64 {

    match s {
        Shape::Circle(x) => 3.14 * x * x,
        Shape::Square(x) => x * x,
        Shape::Rectangle {ht: x, wid: y} => x * y,
    } 
}

fn main() {
    let s1 = Shape::Circle(10.0);
    let s2 = Shape::Square(5.0);
    let s3 = Shape::Rectangle {ht: 10.0, wid: 2.0};

    println!("{:?}", area(s3));
}

     
