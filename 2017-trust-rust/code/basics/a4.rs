
fn add(x: i32, y: i32) -> i32 {
    x + y; //will not compile.
}

fn main() {
    let m = add(1,2);
    println!("{}", m);
}
