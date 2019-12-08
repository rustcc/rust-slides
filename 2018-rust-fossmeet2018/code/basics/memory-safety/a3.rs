
fn main() {
    // type inference in action!
    let v = vec![1, 2, 3, 4];
    let x = 10;

    println!("{:?}", v);
    println!("{:?}", v[0] + x);

    v[0] = 20; // immutable!
}

