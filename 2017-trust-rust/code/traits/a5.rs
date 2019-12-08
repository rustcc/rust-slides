
fn identity<T>(x: T) -> T {
    x
}

fn main() {
    let x = identity(10);
    let y = identity("hello");

    println!("{},{}", x, y);
}
