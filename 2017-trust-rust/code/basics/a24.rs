
fn main() {
    let v = vec![97, 98, 99, 100];
    
    let s = String::from_utf8(v).unwrap();
    println!("{}", s);
}
