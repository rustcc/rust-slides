
#[link(name = "m")]

extern {
    fn fabs(x: f64) -> f64;
}

fn rust_fabs(x: f64) -> f64 {
    unsafe { fabs(x) }
}

fn main() {
    let t = rust_fabs(-10.23);
    println!("{}", t);
}
