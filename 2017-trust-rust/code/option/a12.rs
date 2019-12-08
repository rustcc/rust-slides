
fn main() {
    let mut v = vec![10];
    let m = v.pop(); 
    let t = m.unwrap();
    println!("{}", t);
}
