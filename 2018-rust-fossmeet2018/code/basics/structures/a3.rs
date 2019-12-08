#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let mut p = Point { x : 0, y : 0 };
    p.x = 10;
    println!("{:?}", p);
}

