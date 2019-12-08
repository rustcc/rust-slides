#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

impl Circle {
    fn area(self) -> f64 {
        3.14 * (self.r * self.r)
    }
}

fn main() {
    let c = Circle { x : 0.0, y : 0.0, r : 4.0 };
    println!("area = {}", c.area());

    println!("{:?}", c); // does this compile?
}
