
struct Circle {
    r: f64,
}

struct Square {
    side: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        3.14 * self.r * self.r
    }
}

impl Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    let c = Circle{r: 10.0};
    let d = Square{side: 10.0};
    println!("{}", c.area());
    println!("{}", d.area());
}
