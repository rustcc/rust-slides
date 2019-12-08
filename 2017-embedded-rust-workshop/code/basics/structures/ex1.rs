#[derive(Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    fn add(&self, other: Complex) -> Complex {

   
    }
}

fn main() {
    let c = Complex { re : 1.0, im : 2.0 };
    let d = Complex { re : 3.0, im : 4.0 };

    let result = c.add(d);

    println!("{:?}", result);
} 
