
fn main() {
    let mut v = vec![10, 20];

    let a = v.pop();
    let b = v.pop();
    let c = v.pop();

    println!("{:?}, {:?}, {:?}", a, b, c);
}
