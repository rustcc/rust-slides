
fn identity<T>(x: T) -> T {
    x
}

fn main() {
    let p = identity(10);
    println!("{}", p);
    let q = identity(1.2);
    println!("{}", q);
    
}
