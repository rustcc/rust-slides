
fn swap(p: (i32, f64)) -> (f64, i32) {
    (p.1, p.0)
}

fn main() {
    let r = swap((10, 2.0));
    println!("{:?}", r);
}
