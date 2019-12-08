
trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    r: f64,
}

struct Square {
    side: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.14 * self.r * self.r
    }
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn is_bigger<T1:HasArea,T2:HasArea>(a: T1,  b: T2) -> bool {
    a.area() > b.area()
}

fn main() {
    let c = Circle{r: 10.0};
    let d = Square{side: 10.0};
    println!("{}", c.area());
    println!("{}", d.area());

    println!("{}", is_bigger(c,d));

}
