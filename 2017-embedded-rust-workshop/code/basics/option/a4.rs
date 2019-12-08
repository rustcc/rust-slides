
fn main() {
    let mut v = vec![10];

    let a = v.pop();
    let b = v.pop();
    println!("{}", b.unwrap());
}
