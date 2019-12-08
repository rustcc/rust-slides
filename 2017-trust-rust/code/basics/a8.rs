
fn main() {
    let mut v = vec![1,2,3];
    v.push(10);
    println!("{:?}", v);
    v[0] = 10;   
    println!("{}", v[0]);
}
