
#[link(name = "m")]

extern {
    fn fabs(x: f64) -> f64;
}

fn main() {
    let t = unsafe { fabs(-1.2) };
    println!("{}", t);
}
